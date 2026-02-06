use std::sync::Arc;

use anyhow::bail;
use gen_proto_types::job::v1::Job;
use thiserror::Error;

use crate::core::ports::{monitor::MonitorPort, publisher::PublisherPort};

#[derive(Debug, Error)]
enum JobServiceError {
    #[error("unknown job type supplied")]
    UnknownJobType,
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

    pub async fn process_job(&self, job: &Job) -> anyhow::Result<()> {
        let result = match job.job_type() {
            gen_proto_types::job::v1::JobType::Http => self.http_adapter.execute(job).await?,
            gen_proto_types::job::v1::JobType::Ping => self.ping_adapter.execute(job).await?,
            gen_proto_types::job::v1::JobType::Unspecified => {
                bail!(JobServiceError::UnknownJobType)
            }
        };

        self.publisher_adapter.publish_result(&result).await
    }
}
