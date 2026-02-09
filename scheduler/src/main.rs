use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use config::{PostgresConfig, RabbitMQConfig};
use gen_proto_types::job::v1::{Job, JobType};
use scheduler::{
    adapters::{
        inbound::ticker::SchedulerTicker,
        outbound::{postgres::PostgresFetcher, rabbitmq::RabbitMQPublisher},
    },
    core::{
        ports::{job_fetcher::JobFetcher, publisher::Publisher, ticker::Ticker},
        service::{self, schedulerservice::SchedulerService},
    },
    telemetry,
};
use tokio::{
    signal::unix::SignalKind,
    time::{Interval, sleep},
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    let rabbitmq_config = RabbitMQConfig::from_env(&["RABBITMQ_SCHEDULER_QUEUE"])
        .context("while loading RabbitMQ config from environment")?;
    let db_config =
        PostgresConfig::from_env().context("while loading database config from environment")?;

    let shutdown_token = tokio_util::sync::CancellationToken::new();

    let fetcher = PostgresFetcher::new(&db_config.postgres_connection_url())
        .context("while initializing postgres fetcher")?;

    let publisher = RabbitMQPublisher::new(
        &rabbitmq_config.rabbitmq_connection_url(false),
        rabbitmq_config
            .rabbitmq_queue_name(0)
            .context("while getting scheduler queue name")?,
    )
    .context("while initializing rabbitmq publisher")?;
    publisher
        .setup_schema()
        .await
        .context("while setting up rabbitmq publisher schema")?;

    let service = SchedulerService::new(Arc::from(fetcher), Arc::from(publisher));

    let ticker = SchedulerTicker::new(
        Arc::from(service),
        Duration::from_secs(1),
        shutdown_token.clone(),
    );

    let mut ticker_handle = tokio::spawn(async move {
        ticker.start().await;
    });

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
        _ = &mut ticker_handle => {
            warn!("ticker exited prematurely")
        }
        _ = tokio::signal::ctrl_c() => {
            info!("received ctrl-c. exiting scheduler");
        }
        _ = sigterm.recv() => {
            info!("received SIGTERM. exiting scheduler");
        }
    }

    info!("exiting scheduler");
    shutdown_token.cancel();

    let _ = ticker_handle.await;

    Ok(())
}
