use async_trait::async_trait;
use deadpool_lapin::Pool;
use futures_lite::StreamExt;
use gen_proto_types::job_result::v1::JobResult;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, QueueDeclareOptions},
    types::{
        AMQPValue::{LongInt, LongString},
        FieldTable,
    },
};
use prost::{DecodeError, Message};
use thiserror::Error;
use tracing::{debug, error, info};

use crate::core::{
    domain::errors::JobEvaluatorError, ports::message_consumer::MessageConsumerPort,
    service::evalservice::EvalService,
};

/// Errors for [`RabbitMQDriver`]
#[derive(Debug, Error)]
pub enum RabbitMQDriverError {
    /// Raised when the [`JobResult`] message can't be decoded
    #[error("error decoding job")]
    JobResultDecode(#[from] DecodeError),
    /// Raised when the initial connection to rabbitmq failed
    #[error("connection error")]
    Connection(#[from] lapin::Error),
    /// Raised, when no connection could be obtained from the pool
    #[error("get pool connection error")]
    Pool(#[from] deadpool_lapin::PoolError),
}

/// Allows for converting the RabbitMQ-specific errors to domain errors
impl From<RabbitMQDriverError> for JobEvaluatorError {
    fn from(value: RabbitMQDriverError) -> Self {
        match value {
            RabbitMQDriverError::Connection(e) => Self::Unavailable(e.to_string()),
            RabbitMQDriverError::Pool(e) => Self::Unavailable(e.to_string()),

            other => Self::Internal(other.to_string()),
        }
    }
}

pub struct RabbitMQDriver {
    service: EvalService,
    pool: Pool,
    queue_name: String,
}

impl RabbitMQDriver {
    #[must_use]
    pub const fn new(service: EvalService, pool: Pool, queue_name: String) -> Self {
        Self {
            service,
            pool,
            queue_name,
        }
    }
    /// # Errors
    ///
    /// Will return [`RabbitMQDriverError`] if:
    /// - no connection could be retrieved from the pool
    /// - the channel could not be created
    /// - the queue could not be declared
    pub async fn setup_schema(&self) -> Result<(), RabbitMQDriverError> {
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
        args.insert("x-delivery-limit".into(), LongInt(5));
        // TODO: specify dead letter exchange in setup schema

        args
    }

    /// # Errors
    ///
    /// Will return [`RabbitMQDriverError`] if deliveries could not be `acked`
    /// or `nacked`
    pub async fn start(&self) -> Result<(), RabbitMQDriverError> {
        let connection = self.pool.get().await?;

        let channel = connection.create_channel().await?;

        let mut consumer = channel
            .basic_consume(
                &self.queue_name,
                "consumertag",
                BasicConsumeOptions {
                    no_ack: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        info!("starting consume");

        loop {
            if let Some(delivery_result) = consumer.next().await {
                let delivery = delivery_result?;
                let service = self.service.clone();
                match JobResult::decode(&delivery.data[..]) {
                    Ok(job_result) => {
                        debug!("received job: {:?}", &job_result);
                        match service.evaluate_job_result(&job_result).await {
                            Ok(()) => {
                                debug!("successfully executed job with id {}", &job_result.id);
                                delivery.ack(BasicAckOptions { multiple: false }).await?;
                            }
                            Err(error) => match error {
                                JobEvaluatorError::Internal(e) => {
                                    error!("evaluating job {} failed: {}", &job_result.id, e);
                                    nack_delivery(&delivery, false).await?;
                                }
                                JobEvaluatorError::Unavailable(e) => {
                                    error!(
                                        "evaluating job {} failed because no connection to the database could be made: {}",
                                        &job_result.id, e
                                    );
                                    nack_delivery(&delivery, true).await?;
                                }
                            },
                        }
                    }
                    Err(e) => {
                        error!("{}", RabbitMQDriverError::JobResultDecode(e));
                        nack_delivery(&delivery, true).await?;
                    }
                }
            }
        }
    }
}

/// # Errors
///
/// Will return [`RabbitMQDriverError`] if the delivery  could not be nacked
async fn nack_delivery(delivery: &Delivery, requeue: bool) -> Result<bool, RabbitMQDriverError> {
    delivery
        .nack(BasicNackOptions {
            requeue,
            ..Default::default()
        })
        .await
        .map_err(RabbitMQDriverError::Connection)
}

#[async_trait]
impl MessageConsumerPort for RabbitMQDriver {
    async fn setup(&self) -> Result<(), JobEvaluatorError> {
        self.setup_schema().await.map_err(|err| {
            error!("RabbitMQDriver Error: {:?}", err);
            JobEvaluatorError::from(err)
        })
    }
    async fn start(&self) -> Result<(), JobEvaluatorError> {
        self.start().await.map_err(|err| {
            error!("RabbitMQDriver Error: {:?}", err);
            JobEvaluatorError::from(err)
        })
    }
}
