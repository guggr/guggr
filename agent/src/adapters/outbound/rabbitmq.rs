use std::sync::Arc;

use async_trait::async_trait;
use gen_proto_types::job_result::v1::JobResult;
use lapin::{
    BasicProperties, Channel, Connection,
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable},
};
use prost::Message;
use tracing::debug;

use crate::core::ports::publisher::PublisherPort;

pub struct RabbitMQPublisher {
    channel: Channel,
    queue_name: String,
}

impl RabbitMQPublisher {
    pub async fn new(connection: Arc<Connection>, queue_name: String) -> anyhow::Result<Self> {
        let channel = connection.create_channel().await?;

        let mut args = FieldTable::default();
        args.insert(
            "x-queue-type".into(),
            AMQPValue::LongString("quorum".into()),
        );

        channel
            .queue_declare(
                &queue_name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                args,
            )
            .await?;

        Ok(RabbitMQPublisher {
            channel,
            queue_name,
        })
    }
}

#[async_trait]
impl PublisherPort for RabbitMQPublisher {
    async fn publish_result(&self, job_result: &JobResult) -> anyhow::Result<()> {
        let encoded_job = job_result.encode_to_vec();

        // TODO possibly exchange configuration via ENV
        let confirm = self
            .channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions::default(),
                &encoded_job,
                BasicProperties::default(),
            )
            .await?
            .await?;

        debug!(
            "published message for job result with id {}: confirmation = {:?}",
            job_result.id, confirm
        );

        Ok(())
    }
}
