use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use config::{PostgresConfig, RabbitMQConfig};
use evaluator::{
    adapters::{inbound::rabbitmq::RabbitMQDriver, outbound::postgres::PostgresAdapter},
    core::service::evalservice::EvalService,
    telemetry,
};
use tokio::{select, signal::unix::SignalKind};
use tracing::{debug, error, info};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    debug!("Loading rabbitmq config from env");
    let rabbitmq_config = RabbitMQConfig::from_env(&["RABBITMQ_JOB_RESULT_QUEUE"])
        .context("while loading RabbitMQ config from environment")?;

    debug!("Loading db config from env");
    let db_config =
        PostgresConfig::from_env().context("while loading database config from environment")?;

    debug!("creating rabbitmq pool");
    let rabbitmq_pool =
        evaluator::create_rabbitmq_pool(&rabbitmq_config.rabbitmq_connection_url(false))
            .await
            .map_err(|e| anyhow!(e))?;

    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres_adapter = Arc::from(
        PostgresAdapter::new(&db_config.postgres_connection_url())
            .context("while initializing postgres fetcher")?,
    );

    debug!("initializing service");
    let service = EvalService::new(postgres_adapter);

    debug!("initializing rabbitmq driver");
    let rabbitmq_driver = RabbitMQDriver::new(
        service,
        rabbitmq_pool.clone(),
        rabbitmq_config.rabbitmq_queue_name(0).unwrap(),
    );
    rabbitmq_driver.setup_schema().await?;

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    info!("starting evaluator");

    select! {
        res = rabbitmq_driver.start() => {
            match res {
                Ok(()) => {}
                Err(err) => {
                    error!("error happened while executing evaluator: {}", err);
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("received ctrl-c signal. exiting evaluator");
        }

        _ = sigterm.recv() => {
            info!("received SIGTERM, exiting evaluator");
        }
    }
    rabbitmq_pool.close();
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    Ok(())
}
