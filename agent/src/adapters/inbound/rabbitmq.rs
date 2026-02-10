use deadpool_lapin::Pool;
use futures_lite::StreamExt;
use gen_proto_types::job::v1::Job;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
};
use prost::{DecodeError, Message};
use thiserror::Error;
use tracing::{error, info};

use crate::core::service::jobservice::{JobService, JobServiceError};

#[derive(Debug, Error)]
pub enum RabbitMQDriverError {
    #[error("error decoding job")]
    JobDecode(#[from] DecodeError),
    #[error("connection error")]
    Connection(#[from] lapin::Error),
    #[error("create pool error")]
    CreatePool(#[from] deadpool_lapin::CreatePoolError),
}

pub struct RabbitMQDriver {
    service: JobService,
    pool: Pool,
    queue_name: String,
}

impl RabbitMQDriver {
    pub async fn new(
        pool: Pool,
        queue_name: String,
        service: JobService,
    ) -> Result<Self, RabbitMQDriverError> {
        Ok(RabbitMQDriver {
            pool,
            service,
            queue_name,
        })
    }

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

        info!("starting consume");

        loop {
            match consumer.next().await {
                Some(delivery_result) => {
                    let delivery = delivery_result?;
                    let service = self.service.clone();
                    tokio::spawn(async move {
                        match Job::decode(&delivery.data[..]) {
                            Ok(job) => {
                                info!("received job: {:?}", &job);
                                match service.process_job(&job).await {
                                    Ok(_) => {
                                        info!("successfully executed job with id {}", &job.id);
                                        delivery.ack(BasicAckOptions { multiple: false }).await?;
                                    }
                                    Err(error) => {
                                        match error {
                                            JobServiceError::UnknownJobType => {
                                                error!("executing job {} failed because an unknown job type has been supplied.", &job.id);
                                                nack_delivery(&delivery, false).await?;
                                            }
                                            JobServiceError::AgentIssue(e) => {
                                                error!("executing job {} failed because of an agent issue: {}", &job.id, e);
                                                nack_delivery(&delivery, true).await?;
                                                return Err(e);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("{}", RabbitMQDriverError::JobDecode(e));
                                nack_delivery(&delivery, true).await?;
                            }
                        }
                        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                    }).await??;
                }
                None => continue,
            }
        }
    }
}

async fn nack_delivery(delivery: &Delivery, requeue: bool) -> Result<bool, RabbitMQDriverError> {
    delivery
        .nack(BasicNackOptions {
            requeue,
            ..Default::default()
        })
        .await
        .map_err(RabbitMQDriverError::Connection)
}
