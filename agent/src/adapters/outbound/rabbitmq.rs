use async_trait::async_trait;
use gen_proto_types::job_result::v1::JobResult;
use lapin::{
    BasicProperties, Channel, Connection,
    options::BasicPublishOptions,
    types::{AMQPValue, FieldTable},
};
use prost::Message;
use thiserror::Error;
use tracing::debug;

use crate::core::{ports::publisher::PublisherPort, service::jobservice::JobServiceError};

pub struct RabbitMQPublisher {
    channel: Channel,
    queue_name: String,
}

#[derive(Error, Debug)]
pub enum RabbitMQPublisherError {
    #[error("rabbitmq connection error")]
    ConnectionError(#[from] lapin::Error),
}

impl RabbitMQPublisher {
    pub async fn new(
        connection: &Connection,
        queue_name: String,
    ) -> Result<Self, RabbitMQPublisherError> {
        let channel = connection
            .create_channel()
            .await
            .map_err(RabbitMQPublisherError::ConnectionError)?;

        let mut args = FieldTable::default();
        args.insert(
            "x-queue-type".into(),
            AMQPValue::LongString("quorum".into()),
        );
        args.insert("x-delivery-limit".into(), AMQPValue::LongInt(5));

        Ok(RabbitMQPublisher {
            channel,
            queue_name,
        })
    }
}

#[async_trait]
impl PublisherPort for RabbitMQPublisher {
    async fn publish_result(&self, job_result: &JobResult) -> Result<(), JobServiceError> {
        let encoded_job = job_result.encode_to_vec();

        // TODO possibly exchange configuration via ENV
        self.channel
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
