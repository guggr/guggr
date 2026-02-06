use std::time::Duration;

use anyhow::{Context, Result};
use config::Config;
use gen_proto_types::job::v1::{Job, JobType};
use scheduler::{
    adapters::outbound::rabbitmq::RabbitMQPublisher, core::ports::publisher::Publisher, telemetry,
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    telemetry::init_tracing();

    let config = Config::from_env(&["RABBITMQ_SCHEDULER_QUEUE"])
        .context("while loading config from environment")?;

    let publisher = RabbitMQPublisher::new(
        &config.connection_url(false),
        config
            .rabbitmq_queue_name(0)
            .context("while getting scheduler queue name")?,
    )
    .context("while initializing rabbitmq publisher")?;
    publisher
        .setup_schema()
        .await
        .context("while setting up rabbitmq publisher schema")?;

    publisher
        .publish(Job {
            id: "123".into(),
            job_type: JobType::Http.into(),
            http: Some(gen_proto_types::job::types::v1::HttpJobType {
                url: "test.de".into(),
            }),
            ping: None,
        })
        .await?;

    sleep(Duration::from_secs(30));

    Ok(())
}
