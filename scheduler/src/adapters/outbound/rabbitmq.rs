use async_trait::async_trait;
use deadpool_lapin::Runtime;
use gen_proto_types::job::v1::Job;
use lapin::{self, BasicProperties, options::BasicPublishOptions};
use prost::Message;
use thiserror::Error;
use tracing::{debug, error, info};

use crate::core::{domain::errors::JobSchedulerError, ports::publisher::Publisher};

/// Publisher instance for pushing jobs to a `RabbitMQ` queue. Uses a connection
/// pool for exchange with the `RabbitMQ` instance.
pub struct RabbitMQPublisher {
    pool: deadpool_lapin::Pool,
    queue_name: String,
}

/// Errors for [`RabbitMQPublisher`]
#[derive(Error, Debug)]
pub enum RabbitMQPublisherError {
    /// Raised, when the initial connection to `RabbitMQ` fails more information
    /// see [`deadpool_lapin::CreatePoolError`].
    #[error("RabbitMQ connection failed: {0}")]
    CreateConnectionError(#[from] deadpool_lapin::CreatePoolError),
    /// Raised, when no connection could be obtained from the connection pool.
    #[error("pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] deadpool_lapin::PoolError),
    /// Raised, when there was an error while interacting with `RabbitMQ`.
    #[error("failure while interacting with rabbitmq: {0}")]
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
    /// Creates a new [`RabbitMQPublisher`] instance from a given connection url
    /// and queue name.
    ///
    /// # Errors
    /// Returns an error if the connection pool creation fails.
    pub fn new(connection_url: &str, queue_name: String) -> Result<Self, RabbitMQPublisherError> {
        let config = deadpool_lapin::Config {
            url: Some(connection_url.into()),
            ..Default::default()
        };

        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Self { pool, queue_name })
    }

    /// Publishes a given [`Job`] to the specified queue (through
    /// [`Self::new`]).
    ///
    /// # Errors
    /// Returns an error if
    /// - getting a connection from the Pool fails
    ///   ([`RabbitMQPublisherError::PoolGetConnectionError`])
    /// - creating a new channel fails
    ///   ([`RabbitMQPublisherError::RabbitMQInteractionError`])
    /// - publishing the message fails
    ///   ([`RabbitMQPublisherError::RabbitMQInteractionError`])
    async fn publish_to_queue(&self, job: Job) -> Result<(), RabbitMQPublisherError> {
        debug!("getting connection and channel for RabbitMQ");
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

    /// Closes the connection pool. Required for graceful shutdown.
    pub fn close(&self) {
        info!("closing RabbitMQ connection pool");
        self.pool.close();
    }
}

#[async_trait]
impl Publisher for RabbitMQPublisher {
    async fn publish(&self, job: Job) -> Result<(), JobSchedulerError> {
        Ok(self.publish_to_queue(job).await.map_err(|err| {
            error!("RabbitMQ error: {:?}", err);
            JobSchedulerError::from(err)
        })?)
    }
}
