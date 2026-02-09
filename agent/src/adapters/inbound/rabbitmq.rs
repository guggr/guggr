use std::sync::Arc;

use futures_lite::StreamExt;
use gen_proto_types::job::v1::Job;
use lapin::{
    Channel, Connection, Error,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions},
    types::FieldTable,
};
use prost::{DecodeError, Message};
use thiserror::Error;
use tokio::select;
use tracing::{error, info};

use crate::core::service::jobservice::{JobService, JobServiceError};

#[derive(Debug, Error)]
pub enum RabbitMQDriverError {
    #[error("error decoding job")]
    JobDecodeError(#[from] DecodeError),
    #[error("connection error")]
    ConnectionError(#[from] Error),
}

pub struct RabbitMQDriver {
    service: JobService,
    connection: Arc<Connection>,
    channel: Channel,
    queue_name: String,
}

impl RabbitMQDriver {
    pub async fn new(
        connection: Arc<Connection>,
        queue_name: String,
        service: JobService,
    ) -> Result<Self, RabbitMQDriverError> {
        let channel = connection
            .create_channel()
            .await
            .map_err(RabbitMQDriverError::ConnectionError)?;

        Ok(RabbitMQDriver {
            service,
            connection,
            channel,
            queue_name,
        })
    }

    pub async fn start(&self) -> Result<(), RabbitMQDriverError> {
        let mut consumer = self
            .channel
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
            select! {
                msg = consumer.next() => {
                    match msg {
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
                                                delivery.ack(BasicAckOptions { multiple: false }).await.expect("error while ack'ing delivery");
                                            },
                                            Err(error) => {
                                                match error {
                                                    JobServiceError::UnknownJobType => {
                                                        error!("executing job {} failed because an unknown job type has been supplied.", &job.id);
                                                        nack_delivery(&delivery, false).await;
                                                    }
                                                    JobServiceError::AgentIssue(e) => {
                                                        error!("executing job {} failed because of an agent issue: {}", &job.id, e);
                                                        nack_delivery(&delivery, true).await;
                                                        error!("exiting agent due to agent issues in previous jobs");
                                                        std::process::exit(1);
                                                    }
                                                }
                                            },
                                        }
                                    },
                                    Err(e) => {
                                        error!("{}", RabbitMQDriverError::JobDecodeError(e));
                                        nack_delivery(&delivery, true).await;
                                    },
                                }
                            });

                        },
                        None => break,
                    }
                }

                _ = tokio::signal::ctrl_c() => {
                    info!("received ctrl-c signal. exiting agent");
                    self.connection.close(0, "consumer closed connection due to exit").await?;
                }
            }
        }

        Ok(())
    }
}

async fn nack_delivery(delivery: &Delivery, requeue: bool) {
    delivery
        .nack(BasicNackOptions {
            requeue,
            ..Default::default()
        })
        .await
        .expect("error while sending nack");
}
