use agent::generate_run_id;
use deadpool_lapin::Pool;
use futures_lite::StreamExt;
use gen_proto_types::job::v1::Job;
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
use tracing::{error, info};

use crate::core::service::jobservice::{JobService, JobServiceError};

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
        args.insert("delivery-limit".into(), LongInt(5));
        // TODO: specify dead letter exchange in setup schema

        args
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
            if let Some(delivery_result) = consumer.next().await {
                let delivery = delivery_result?;
                let service = self.service.clone();
                let run_id = generate_run_id();
                tokio::spawn(async move {
                    process_delivery(delivery, service, run_id).await?;

                    Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                })
                .await??;
            }
        }
    }
}

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

async fn nack_delivery(delivery: &Delivery, requeue: bool) -> Result<bool, RabbitMQDriverError> {
    delivery
        .nack(BasicNackOptions {
            requeue,
            ..Default::default()
        })
        .await
        .map_err(RabbitMQDriverError::Connection)
}
