use std::{collections::HashMap, error::Error, sync::Arc};

use config::{AgentConfig, RabbitMQConfig};
use gen_proto_types::job::v1::JobType;
use tokio::{select, signal::unix::SignalKind};
use tracing::{error, info};

use crate::{
    adapters::{
        inbound::rabbitmq::RabbitMQDriver,
        outbound::{http::HttpAdapter, ping::PingAdapter, rabbitmq::RabbitMQPublisher},
    },
    core::{
        ports::monitor::MonitorPort,
        service::jobservice::{AgentError, JobService},
    },
};

mod adapters;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    agent::init_tracing();

    let rabbit_mq_config =
        RabbitMQConfig::from_env(&["RABBITMQ_JOBS_QUEUE", "RABBITMQ_JOB_RESULT_QUEUE"])?;
    let agent_config = AgentConfig::from_env();

    let rabbitmq_pool =
        agent::create_rabbitmq_pool(&rabbit_mq_config.rabbitmq_connection_url(false)).await?;

    // Outbound adapter
    let http_adapter = Arc::new(HttpAdapter::new(
        agent_config.http_backup_endpoint().clone(),
    ));
    let ping_adapter = Arc::new(PingAdapter::new(
        agent_config.ping_backup_endpoint().clone(),
    ));
    let rabbitmq_publisher = Arc::new(RabbitMQPublisher::new(
        rabbitmq_pool.clone(),
        rabbit_mq_config.rabbitmq_queue_name(1).unwrap(),
    ));

    rabbitmq_publisher.setup_queue().await?;

    let mut processing_adapter: HashMap<JobType, Arc<dyn MonitorPort + Send + Sync>> =
        HashMap::new();
    processing_adapter.insert(JobType::Http, http_adapter);
    processing_adapter.insert(JobType::Ping, ping_adapter);

    // Service
    let job_service = JobService::new(processing_adapter, rabbitmq_publisher);

    // Inbound adapter
    let rabbitmq_driver = RabbitMQDriver::new(
        rabbitmq_pool.clone(),
        rabbit_mq_config.rabbitmq_queue_name(0).unwrap(),
        job_service,
    );
    rabbitmq_driver.setup_queues().await?;

    info!("Agent is starting...");

    let mut sigterm = tokio::signal::unix::signal(SignalKind::terminate())?;

    select! {
        res = rabbitmq_driver.start() => {
            match res {
                Ok(()) => {}
                Err(err) if err.downcast_ref::<AgentError>().is_some() => {
                    error!("exiting agent due to agent issues in previous jobs");
                    rabbitmq_pool.close();
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    std::process::exit(1);
                }
                Err(err) => {
                    error!("error happened while executing agent: {}", err);
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("received ctrl-c signal. exiting agent");
            rabbitmq_pool.close();
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        _ = sigterm.recv() => {
            info!("received SIGTERM, exiting agent");
            rabbitmq_pool.close();
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    }

    Ok(())
}
