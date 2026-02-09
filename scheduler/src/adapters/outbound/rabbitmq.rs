use async_trait::async_trait;
use deadpool_lapin::Runtime;
use gen_proto_types::job::v1::Job;
use lapin::{
    self, BasicProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{AMQPValue::LongString, FieldTable},
};
use prost::Message;
use thiserror::Error;
use tracing::{debug, error, info};

use crate::core::{domain::errors::JobSchedulerError, ports::publisher::Publisher};

pub struct RabbitMQPublisher {
    pool: deadpool_lapin::Pool,
    queue_name: String,
}

#[derive(Error, Debug)]
pub enum RabbitMQPublisherError {
    #[error("RabbitMQ Connection failed: {0}")]
    CreateConnectionError(#[from] deadpool_lapin::CreatePoolError),
    #[error("Pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] deadpool_lapin::PoolError),
    #[error("Failure while interacting with rabbitmq")]
    RabbitMQInteractionError(#[from] lapin::Error),
}

impl From<RabbitMQPublisherError> for JobSchedulerError {
    fn from(value: RabbitMQPublisherError) -> Self {
        match value {
            RabbitMQPublisherError::CreateConnectionError(e) => {
                Self::QueueUnavailable(e.to_string())
            }
            RabbitMQPublisherError::PoolGetConnectionError(e) => {
                Self::QueueUnavailable(e.to_string())
            }
            other @ RabbitMQPublisherError::RabbitMQInteractionError(..) => {
                Self::Internal(other.to_string())
            }
        }
    }
}

impl RabbitMQPublisher {
    pub fn new(connection_url: &str, queue_name: String) -> Result<Self, RabbitMQPublisherError> {
        let config = deadpool_lapin::Config {
            url: Some(connection_url.into()),
            ..Default::default()
        };

        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool, queue_name })
    }

    pub async fn setup_schema(&self) -> Result<(), RabbitMQPublisherError> {
        let connection = self.pool.get().await?;

        let channel = connection.create_channel().await?;

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

    async fn publish_to_queue(&self, job: Job) -> Result<(), RabbitMQPublisherError> {
        debug!("getting connection and channel for rabbitmq");
        let connection = self.pool.get().await?;
        let channel = connection.create_channel().await?;

        let properties = BasicProperties::default()
            .with_content_type("application/protobuf".into())
            .with_delivery_mode(1); // Transient, i.e. not written to disk and not surviving broker restarts

        debug!("publishing to channel");
        channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions::default(),
                &job.encode_to_vec(),
                properties,
            )
            .await?
            .await?;

        Ok(())
    }

    pub fn close(&self) {
        info!("Closing RabbitMQ connection pool");
        self.pool.close();
    }
}

#[async_trait]
impl Publisher for RabbitMQPublisher {
    async fn publish(&self, job: Job) -> Result<(), JobSchedulerError> {
        Ok(self.publish_to_queue(job).await.map_err(|err| {
            error!("RabbitMQ Error: {:?}", err);
            JobSchedulerError::from(err)
        })?)
    }
}
