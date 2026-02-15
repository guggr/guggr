use async_trait::async_trait;
use deadpool_lapin::Pool;
use gen_proto_types::job_result::v1::JobResult;
use lapin::{
    BasicProperties,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{
        AMQPValue::{LongInt, LongString},
        FieldTable,
    },
};
use prost::Message;
use thiserror::Error;
use tracing::debug;

use crate::{
    adapters::inbound::rabbitmq::RabbitMQDriverError,
    core::{
        ports::publisher::PublisherPort,
        service::jobservice::{JobServiceError, JobServiceError::AgentIssue},
    },
};

pub struct RabbitMQPublisher {
    pool: Pool,
    queue_name: String,
}

#[derive(Error, Debug)]
pub enum RabbitMQPublisherError {
    #[error("rabbitmq connection error")]
    Connection(#[from] lapin::Error),
    #[error("pooling error")]
    Pool(#[from] deadpool_lapin::PoolError),
}

impl RabbitMQPublisher {
    pub const fn new(pool: Pool, queue_name: String) -> Self {
        Self { pool, queue_name }
    }

    /// Sets up the `RabbitMQ` queue for publishing job results.
    ///
    /// # Errors
    /// Raises an [`RabbitMQDriverError`] if there is either a problem with
    /// acquiring a connection from the pool, creating a channel on the
    /// connection or declaring the queue.
    pub async fn setup_queue(&self) -> Result<(), RabbitMQDriverError> {
        let connection = self.pool.get().await?;

        let channel = connection.create_channel().await?;

        channel
            .queue_declare(
                &self.queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                Self::queue_args(),
            )
            .await?;

        Ok(())
    }

    fn queue_args() -> FieldTable {
        let mut args = FieldTable::default();
        // set queue type to quorum
        args.insert("x-queue-type".into(), LongString("quorum".into()));
        // set maximum delivery limit until messages get pushed into dead letter
        // exchange
        args.insert("delivery-limit".into(), LongInt(5));
        // TODO: specify dead letter exchange in setup schema

        args
    }
}

#[async_trait]
impl PublisherPort for RabbitMQPublisher {
    /// Publishes the given [`JobResult`] to the corresponding `RabbitMQ` queue.
    ///
    /// # Errors
    /// Raises a [`JobServiceError`] when there is a problem with
    /// - getting a connection from the `RabbitMQ` pool
    /// - creating a channel on the `RabbitMQ` connection
    /// - publishing the result to the `RabbitMQ` queue
    async fn publish_result(&self, job_result: &JobResult) -> Result<(), JobServiceError> {
        let encoded_job = job_result.encode_to_vec();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| AgentIssue(RabbitMQPublisherError::Pool(e).into()))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| AgentIssue(RabbitMQDriverError::Connection(e).into()))?;

        // TODO possibly exchange configuration via ENV
        channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions::default(),
                &encoded_job,
                BasicProperties::default(),
            )
            .await
            .map_err(|e| JobServiceError::AgentIssue(e.into()))?
            .await
            .map_err(|e| JobServiceError::AgentIssue(e.into()))?;

        debug!("published message for job result with id {}", job_result.id);

        Ok(())
    }
}
