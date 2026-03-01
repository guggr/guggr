use std::{sync::Arc, time::Duration};

use anyhow::{Context, Result};
use config::{PostgresConfig, RabbitMQConfig};
use scheduler::{
    adapters::{
        inbound::ticker::SchedulerTicker,
        outbound::{
            postgres::PostgresFetcher, postgres_token_cleaner::PostgresTokenCleaner,
            rabbitmq::RabbitMQPublisher,
        },
    },
    core::{
        ports::ticker::Ticker,
        service::{cleanupservice::CleanupService, schedulerservice::SchedulerService},
    },
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

    debug!("initializing scheduler service");
    let scheduler_service = Arc::from(SchedulerService::new(fetcher, publisher.clone()));

    debug!("initializing scheduler ticker");
    let scheduler_ticker = SchedulerTicker::new(
        scheduler_service,
        Duration::from_secs(1),
        shutdown_token.clone(),
    );

    debug!("initializing postgres token cleaner");
    let token_cleaner = Arc::from(
        PostgresTokenCleaner::new(&db_config.connection_url())
            .context("while initializing postgres token cleaner")?,
    );

    debug!("initializing cleanup service");
    let cleanup_service = Arc::from(CleanupService::new(token_cleaner));

    debug!("initializing cleanup ticker");
    let cleanup_ticker = SchedulerTicker::new(
        cleanup_service,
        Duration::from_secs(60 * 60),
        shutdown_token.clone(),
    );

    info!("adapters and services setup completed");

    debug!("starting scheduler ticker in own task");
    let mut scheduler_handle = tokio::spawn(async move {
        scheduler_ticker.start().await;
    });

    debug!("starting cleanup ticker in own task");
    let mut cleanup_handle = tokio::spawn(async move {
        cleanup_ticker.start().await;
    });

    info!("tickers started");

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
        _ = &mut scheduler_handle => {
            warn!("scheduler ticker exited prematurely");
        }
        _ = &mut cleanup_handle => {
            warn!("cleanup ticker exited prematurely");
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

    let _ = scheduler_handle.await;
    let _ = cleanup_handle.await;

    publisher.close();
    // Do not remove. Publisher close requires some millis for closing connections
    // gracefully. Otherwise, errors will be at the end of the log file.
    tokio::time::sleep(Duration::from_millis(100)).await;

    Ok(())
}
