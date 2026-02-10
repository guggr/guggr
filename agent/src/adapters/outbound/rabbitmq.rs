use async_trait::async_trait;
use deadpool_lapin::Pool;
use gen_proto_types::job_result::v1::JobResult;
use lapin::{BasicProperties, options::BasicPublishOptions};
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
    ConnectionError(#[from] lapin::Error),
    #[error("pooling error")]
    PoolError(#[from] deadpool_lapin::PoolError),
}

impl RabbitMQPublisher {
    pub async fn new(pool: Pool, queue_name: String) -> Result<Self, RabbitMQPublisherError> {
        Ok(RabbitMQPublisher { pool, queue_name })
    }
}

#[async_trait]
impl PublisherPort for RabbitMQPublisher {
    async fn publish_result(&self, job_result: &JobResult) -> Result<(), JobServiceError> {
        let encoded_job = job_result.encode_to_vec();

        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| AgentIssue(RabbitMQPublisherError::PoolError(e).into()))?;

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
