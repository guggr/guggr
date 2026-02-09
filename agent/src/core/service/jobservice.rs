use std::{error::Error, sync::Arc};

use gen_proto_types::job::v1::Job;
use thiserror::Error;

use crate::core::ports::{monitor::MonitorPort, publisher::PublisherPort};

#[derive(Debug, Error)]
pub enum JobServiceError {
    #[error("unknown job type supplied")]
    UnknownJobType,
    #[error("issue with the agent")]
    AgentIssue(#[from] Box<dyn Error + Send + Sync>),
}

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("error with http request")]
    Http(#[from] reqwest::Error),
    #[error("could not retrieve remote address")]
    RemoteAddress,
    #[error("error with icmp request")]
    Ping(#[from] Box<dyn Error + Send + Sync>),
}

pub struct JobService {
    http_adapter: Arc<dyn MonitorPort + Send + Sync>,
    ping_adapter: Arc<dyn MonitorPort + Send + Sync>,
    publisher_adapter: Arc<dyn PublisherPort + Send + Sync>,
}

impl Clone for JobService {
    fn clone(&self) -> Self {
        Self {
            http_adapter: Arc::clone(&self.http_adapter),
            ping_adapter: Arc::clone(&self.ping_adapter),
            publisher_adapter: Arc::clone(&self.publisher_adapter),
        }
    }
}

impl JobService {
    pub fn new(
        http_adapter: Arc<dyn MonitorPort + Send + Sync>,
        ping_adapter: Arc<dyn MonitorPort + Send + Sync>,
        publisher_adapter: Arc<dyn PublisherPort + Send + Sync>,
    ) -> Self {
        JobService {
            http_adapter,
            ping_adapter,
            publisher_adapter,
        }
    }

    pub async fn process_job(&self, job: &Job) -> Result<(), JobServiceError> {
        let result = match job.job_type() {
            gen_proto_types::job::v1::JobType::Http => self.http_adapter.execute(job).await?,
            gen_proto_types::job::v1::JobType::Ping => self.ping_adapter.execute(job).await?,
            gen_proto_types::job::v1::JobType::Unspecified => {
                return Err(JobServiceError::UnknownJobType);
            }
        };

        self.publisher_adapter.publish_result(&result).await
    }
}
