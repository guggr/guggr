use std::sync::Arc;

use agent::generate_run_id;
use deadpool_lapin::Pool;
use futures_util::StreamExt;
use gen_proto_types::job::v1::Job;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
};
use prost::{DecodeError, Message};
use thiserror::Error;
use tokio::sync::{Semaphore, mpsc};
use tracing::{error, info};

use crate::core::service::jobservice::{AgentError, JobService, JobServiceError};

#[derive(Debug, Error)]
pub enum RabbitMQDriverError {
    #[error("error decoding job")]
    JobDecode(#[from] DecodeError),
    #[error("connection error")]
    Connection(#[from] lapin::Error),
    #[error("get pool connection error")]
    Pool(#[from] deadpool_lapin::PoolError),
}

pub struct RabbitMQDriver {
    service: JobService,
    pool: Pool,
    queue_name: String,
}

impl RabbitMQDriver {
    pub const fn new(pool: Pool, queue_name: String, service: JobService) -> Self {
        Self {
            service,
            pool,
            queue_name,
        }
    }

    /// Starts consuming the jobs from the `RabbitMQ` job queue and processing
    /// them.
    ///
    /// # Errors
    /// Raises an error if there was a problem with
    /// - retrieving a connection from the `RabbitMQ` pool
    /// - with creating a channel
    /// - with creating a consumer on the channel
    /// - owning a permit of the Semaphore limiting the concurrent tasks
    /// - the received delivery
    /// - processing the delivery
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

        // Async limitations
        let (tx, mut rx) = mpsc::channel(50);
        // Limit to 20 tasks simultaneously
        let semaphore = Arc::new(Semaphore::new(20));

        info!("starting consume");

        loop {
            tokio::select! {
                // Read messages, waits if semaphore is full
                permit = semaphore.clone().acquire_owned() => {
                    let permit = permit?;

                    if let Some(delivery_result) = consumer.next().await {
                        let delivery = delivery_result?;
                        let service = self.service.clone();
                        let run_id = generate_run_id();

                        let tx_clone = tx.clone();

                        tokio::spawn(async move {

                            // move into task, is freed when dropped at the end of the task
                            let _permit = permit;

                            if let Err(e) = process_delivery(delivery, service, run_id).await {
                                // Send error to main task
                                let _ = tx_clone.send(e).await;
                            }
                        });
                    } else {
                        break;
                    }
                }
                // Collect errors
                Some(err) = rx.recv() => {
                    if err.downcast_ref::<AgentError>().is_some() {
                        return Err(err)
                    }
                    error!("error happened while processing delivery: {}", err);
                }

            }
        }
        Ok(())
    }
}

/// Decodes a given delivery and hands the job over to the [`JobService`] to
/// process the job.
///
/// # Errors
/// Raises an error if there was a problem with
/// - decoding the delivery into a job [`Job`]
/// - The processing of the job throws an error. For more information see
///   [`JobService::process_job`]
#[tracing::instrument(skip(delivery, service), fields(run_id = %run_id))]
async fn process_delivery(
    delivery: Delivery,
    service: JobService,
    run_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match Job::decode(&delivery.data[..]) {
        Ok(job) => {
            info!("received job: {:?}", &job);
            match service.process_job(&job, run_id).await {
                Ok(()) => {
                    info!("successfully executed job");
                    delivery.ack(BasicAckOptions { multiple: false }).await?;
                    Ok(())
                }
                Err(error) => match error {
                    JobServiceError::UnknownJobType => {
                        error!(
                            "executing job failed because an unknown job type has been supplied."
                        );
                        nack_delivery(&delivery, false).await?;
                        Err(error.into())
                    }
                    JobServiceError::AgentIssue(e) => {
                        error!("executing job failed because of an agent issue: {}", e);
                        nack_delivery(&delivery, true).await?;
                        Err(e)
                    }
                },
            }
        }
        Err(e) => {
            let decode_err = RabbitMQDriverError::JobDecode(e);
            error!("{}", decode_err);
            nack_delivery(&delivery, true).await?;
            Err(decode_err.into())
        }
    }
}

/// 'Nacks' a delivery
///
/// # Errors
/// Raises an [`RabbitMQError`] when there is a problem with the `RabbitMQ`
/// connection.
async fn nack_delivery(delivery: &Delivery, requeue: bool) -> Result<bool, RabbitMQDriverError> {
    delivery
        .nack(BasicNackOptions {
            requeue,
            ..Default::default()
        })
        .await
        .map_err(RabbitMQDriverError::Connection)
}
