use std::{sync::Arc, time::Duration};

use config::Config;
use lapin::{Connection, ConnectionProperties};
use tokio::time::sleep;
use tracing::{error, info, warn};
use tracing_subscriber::FmtSubscriber;

use crate::{
    adapters::{
        inbound::rabbitmq::RabbitMQDriver,
        outbound::{http::HttpAdapter, ping::PingAdapter, rabbitmq::RabbitMQPublisher},
    },
    core::service::jobservice::JobService,
};

mod adapters;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_filename(".env.sample").ok();
    let subscriber = FmtSubscriber::builder().finish();

    tracing::subscriber::set_global_default(subscriber).expect("error setting global subscriber");

    let config = Config::from_env(&["RABBITMQ_JOBS_QUEUE", "RABBITMQ_JOB_RESULT_QUEUE"])?;

    let mut retry_count = 0;

    let connection = loop {
        match Connection::connect(
            &config.connection_url(false),
            ConnectionProperties::default(),
        )
        .await
        {
            Ok(conn) => {
                info!("successfully connected to rabbitmq host");
                break Arc::new(conn);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count > 5 {
                    error!("error connecting to rabbitmq after 5 retries: {}", e);
                    std::process::exit(1);
                }

                warn!(
                    "temporary error connecting to rabbitmq (try {}/5). retrying...",
                    retry_count
                );
                sleep(Duration::from_secs(10)).await;
            }
        }
    };

    // Outbound adapter
    let http_adapter = Arc::new(HttpAdapter::new());
    let ping_adapter = Arc::new(PingAdapter::new());
    let rabbitmq_publisher = Arc::new(
        RabbitMQPublisher::new(connection.clone(), config.rabbitmq_queue_name(1).unwrap()).await?,
    );

    // Service
    let job_service = JobService::new(http_adapter, ping_adapter, rabbitmq_publisher);

    // Inbound adapter
    let rabbitmq_driver = RabbitMQDriver::new(
        connection.clone(),
        config.rabbitmq_queue_name(0).unwrap(),
        job_service,
    )
    .await?;

    rabbitmq_driver.start().await?;

    Ok(())
}
