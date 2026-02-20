use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use config::{PostgresConfig, RabbitMQConfig};
use scheduler::{
    adapters::{
        inbound::ticker::SchedulerTicker,
        outbound::{postgres::PostgresFetcher, rabbitmq::RabbitMQPublisher},
    },
    core::{ports::ticker::Ticker, service::schedulerservice::SchedulerService},
    telemetry,
};
use tokio::signal::unix::SignalKind;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    debug!("Loading rabbitmq config from env");
    let rabbitmq_config = RabbitMQConfig::from_env(&["RABBITMQ_JOBS_QUEUE"])
        .context("while loading RabbitMQ config from environment")?;
    debug!("Loading db config from env");
    let db_config =
        PostgresConfig::from_env().context("while loading database config from environment")?;

    let shutdown_token = tokio_util::sync::CancellationToken::new();

    debug!("initializing postgres fetcher and running pending migrations on the database");
    let fetcher = Arc::from(
        PostgresFetcher::new(&db_config.connection_url())
            .context("while initializing postgres fetcher")?,
    );

    debug!("initializing publisher");
    let publisher = Arc::from(
        RabbitMQPublisher::new(
            &rabbitmq_config.connection_url(false),
            rabbitmq_config
                .queue_name(0)
                .context("while getting scheduler queue name")?,
        )
        .context("while initializing rabbitmq publisher")?,
    );
    debug!("setting publisher schema up");
    publisher
        .setup_schema()
        .await
        .context("while setting up rabbitmq publisher schema")?;

    debug!("initializing service");
    let service = SchedulerService::new(fetcher, publisher.clone());

    debug!("initializing ticker");
    let ticker = SchedulerTicker::new(
        Arc::from(service),
        Duration::from_secs(1),
        shutdown_token.clone(),
    );

    info!("adapters and service setup completed");

    debug!("starting ticker in own task");
    let mut ticker_handle = tokio::spawn(async move {
        ticker.start().await;
    });

    info!("ticker started");

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
        _ = &mut ticker_handle => {
            warn!("ticker exited prematurely");
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

    publisher.close();
    // Do not remove. Publisher close requires some millis for closing connections
    // gracefully. Otherwise, errors will be at the end of the log file.
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(())
}
