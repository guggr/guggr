use std::{error::Error, sync::Arc, time::Duration};

use config::RabbitMQConfig;
use lapin::{Connection, ConnectionProperties};
use tokio::{select, time::sleep};
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    adapters::{
        inbound::rabbitmq::RabbitMQDriver,
        outbound::{http::HttpAdapter, ping::PingAdapter, rabbitmq::RabbitMQPublisher},
    },
    core::service::jobservice::{AgentError, JobService},
};

mod adapters;
mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();

    let config = RabbitMQConfig::from_env(&["RABBITMQ_JOBS_QUEUE", "RABBITMQ_JOB_RESULT_QUEUE"])?;

    let connection = agent::connect_rabbitmq(config.rabbitmq_connection_url(false)).await?;

    // Outbound adapter
    let http_adapter = Arc::new(HttpAdapter::new());
    let ping_adapter = Arc::new(PingAdapter::new());
    let rabbitmq_publisher = Arc::new(
        RabbitMQPublisher::new(&connection, config.rabbitmq_queue_name(1).unwrap()).await?,
    );

    // Service
    let job_service = JobService::new(http_adapter, ping_adapter, rabbitmq_publisher);

    // Inbound adapter
    let rabbitmq_driver = RabbitMQDriver::new(
        &connection,
        config.rabbitmq_queue_name(0).unwrap(),
        job_service,
    )
    .await?;

    select! {
        res = rabbitmq_driver.start() => {
            match res {
                Ok(_) => {}
                Err(err) if err.downcast_ref::<AgentError>().is_some() => {
                    error!("exiting agent due to agent issues in previous jobs");
                    connection
                        .close(1, "connection closed due to agent issue")
                        .await?;
                    std::process::exit(1);
                }
                Err(err) => {
                    error!("error happened while executing agent: {}", err)
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("received ctrl-c signal. exiting agent");
            connection.close(0, "consumer closed connection due to exit").await?;
        }
    }

    Ok(())
}

pub fn init_tracing() {
    let fmt_layer = fmt::layer().with_file(true).with_line_number(true).json(); // Keep JSON for production logs

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init(); // .init() is shorthand for set_global_default
}
