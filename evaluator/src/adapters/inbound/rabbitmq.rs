use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use deadpool_lapin::Pool;
use futures_lite::StreamExt;
use gen_proto_types::{job::v1::Job, job_result::v1::JobResult};
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicQosOptions},
    types::FieldTable,
};
use prost::{DecodeError, Message};
use thiserror::Error;
use tracing::{debug, error, info};

use crate::{
    ToProto,
    core::{
        domain::errors::JobEvaluatorError, ports::message_consumer::MessageConsumerPort,
        service::evalservice::EvalService,
    },
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

    /// Starts the `RabbitMQ` consumption and evaluation of retrieved jobs
    ///
    /// # Errors
    ///
    /// Will return [`RabbitMQDriverError`] if deliveries could not be `acked`
    /// or `nacked`
    pub async fn start(&self) -> Result<(), RabbitMQDriverError> {
        let connection = self.pool.get().await?;

        let channel = connection.create_channel().await?;
        // Cap at 10 max messages
        channel.basic_qos(10, BasicQosOptions::default()).await?;

        let service = Arc::new(self.service.clone());

        let queues = vec![&self.queue_name, "dlx.queue"];

        for queue_name in queues {
            let channel = channel.clone();
            let queue_name = queue_name.to_string();
            let service_clone = service.clone();

            tokio::spawn(async move {
                loop {
                    let consumer_result = channel
                        .basic_consume(
                            &queue_name,
                            Default::default(),
                            BasicConsumeOptions {
                                no_ack: false,
                                ..Default::default()
                            },
                            FieldTable::default(),
                        )
                        .await;

                    info!("Starting consume for queue: {}", queue_name);

                    let mut consumer = match consumer_result {
                        Ok(c) => c,
                        Err(e) => {
                            error!(
                                "Failed to setup consumer for {}: {}. Retrying in 5s...",
                                queue_name, e
                            );
                            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                            continue;
                        }
                    };

                    while let Some(delivery_result) = consumer.next().await {
                        match delivery_result {
                            Ok(delivery) => {
                                let service = service_clone.clone();
                                let q_name = queue_name.clone();

                                tokio::spawn(async move {
                                    if let Err(e) =
                                        process_delivery(delivery, q_name, service).await
                                    {
                                        error!("Error in worker task: {:?}", e);
                                    }
                                });
                            }
                            Err(e) => error!("Consumer stream error: {}", e),
                        }
                    }
                }
            });
        }

        std::future::pending::<()>().await;
        Ok(())
    }
}
/// Sends a Negative Acknowledgment (NACK) and conditionally requeues the
/// message
///
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

async fn process_delivery(
    delivery: Delivery,
    queue_name: String,
    service: Arc<EvalService>,
) -> Result<(), RabbitMQDriverError> {
    let job_result: JobResult = if queue_name == "dlx.queue" {
        let job_request = Job::decode(&delivery.data[..])?;
        debug!("found job request in dlx: {:?}", &job_request);

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0));

        JobResult {
            id: job_request.id,
            timestamp: Some(timestamp.to_proto()),
            batch_id: job_request.batch_id,
            run_id: String::from("0"),
            job_type: job_request.job_type,
            ..Default::default()
        }
    } else {
        match JobResult::decode(&delivery.data[..]) {
            Ok(res) => res,
            Err(e) => {
                nack_delivery(&delivery, true).await?;
                return Err(e.into());
            }
        }
    };

    debug!("received job: {:?}", &job_result);

    // Evaluierung
    match service.evaluate_job_result(&job_result).await {
        Ok(()) => {
            debug!("successfully evaluated job with id {}", &job_result.id);
            delivery.ack(BasicAckOptions { multiple: false }).await?;
        }
        Err(error) => {
            handle_evaluator_error(&delivery, &job_result.id, error).await?;
        }
    }

    Ok(())
}

async fn handle_evaluator_error(
    delivery: &Delivery,
    job_id: &str,
    error: JobEvaluatorError,
) -> Result<(), RabbitMQDriverError> {
    match error {
        JobEvaluatorError::Internal(e) => {
            error!("evaluating job {} failed (internal error): {}", job_id, e);
            nack_delivery(delivery, false).await?;
        }
        JobEvaluatorError::Unavailable(e) => {
            error!("evaluating job {} failed: {}", job_id, e);
            nack_delivery(delivery, true).await?;
        }
    }
    Ok(())
}

#[async_trait]
impl MessageConsumerPort for RabbitMQDriver {
    async fn setup(&self) -> Result<(), JobEvaluatorError> {
        Ok(())
    }
    async fn start(&self) -> Result<(), JobEvaluatorError> {
        self.start().await.map_err(|err| {
            error!("RabbitMQDriver Error: {:?}", err);
            JobEvaluatorError::from(err)
        })
    }
}
