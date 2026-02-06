use anyhow::{Context, Result};
use async_trait::async_trait;
use deadpool_lapin::Runtime;
use gen_proto_types::job::v1::Job;
use lapin::{
    self, BasicProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{AMQPValue::LongString, FieldTable},
};
use prost::Message;

use crate::core::ports::publisher::Publisher;

pub struct RabbitMQPublisher {
    pool: deadpool_lapin::Pool,
    queue_name: String,
}

impl RabbitMQPublisher {
    pub fn new(connection_url: &str, queue_name: String) -> Result<Self> {
        let config = deadpool_lapin::Config {
            url: Some(connection_url.into()),
            ..Default::default()
        };

        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool, queue_name })
    }

    pub async fn setup_schema(&self) -> Result<()> {
        let connection = self
            .pool
            .get()
            .await
            .context("while getting connection from pool")?;

        let channel = connection
            .create_channel()
            .await
            .context("while creating channel for given connection")?;

        channel
            .queue_declare(
                &self.queue_name,
                QueueDeclareOptions {
                    durable: true, // not deleted
                    ..Default::default()
                },
                Self::quorum_args(),
            )
            .await?;

        Ok(())
    }

    fn quorum_args() -> FieldTable {
        let mut arguments = FieldTable::default();
        arguments.insert("x-queue-type".into(), LongString("quorum".into()));

        arguments
    }
}

#[async_trait]
impl Publisher for RabbitMQPublisher {
    async fn publish(&self, job: Job) -> Result<()> {
        let connection = self
            .pool
            .get()
            .await
            .context("while getting connection from pool")?;
        let channel = connection
            .create_channel()
            .await
            .context("while creating channel for given connection")?;

        let properties = BasicProperties::default()
            .with_content_type("application/protobuf".into())
            .with_delivery_mode(1); // Transient, i.e. not written to disk and not surviving broker restarts

        channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions {
                    ..Default::default()
                },
                &job.encode_to_vec(),
                properties,
            )
            .await
            .context("while awaiting publishing")?
            .await
            .context("while awaiting publish confirmation")?;

        Ok(())
    }
}
