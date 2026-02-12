use std::{collections::HashMap, error::Error, sync::Arc};

use gen_proto_types::job::v1::{Job, JobType};
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
    processing_adapter: HashMap<JobType, Arc<dyn MonitorPort + Send + Sync>>,
    publisher_adapter: Arc<dyn PublisherPort + Send + Sync>,
}

impl Clone for JobService {
    fn clone(&self) -> Self {
        Self {
            processing_adapter: self.processing_adapter.clone(),
            publisher_adapter: Arc::clone(&self.publisher_adapter),
        }
    }
}

impl JobService {
    pub fn new(
        processing_adapter: HashMap<JobType, Arc<dyn MonitorPort + Send + Sync>>,
        publisher_adapter: Arc<dyn PublisherPort + Send + Sync>,
    ) -> Self {
        Self {
            processing_adapter,
            publisher_adapter,
        }
    }

    /// Takes a job and hands it to the responsible adapter for execution.
    ///
    /// # Errors
    /// Raises an Error if the execution of the job fails within the adapter or
    /// if there is an error while publishing the result back to `RabbitMQ`.
    pub async fn process_job(&self, job: &Job, run_id: String) -> Result<(), JobServiceError> {
        let result = match self.processing_adapter.get(&job.job_type()) {
            None => return Err(JobServiceError::UnknownJobType),
            Some(processing_adapter) => processing_adapter.execute(job, run_id).await?,
        };

        self.publisher_adapter.publish_result(&result).await
    }
}
