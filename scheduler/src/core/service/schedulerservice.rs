use std::sync::Arc;

use gen_proto_types::job::v1::{Job, JobType};
use tokio_util::task::TaskTracker;
use tracing::{debug, error};

use crate::core::{
    domain::{errors::JobSchedulerError, type_mapper::FromDatabaseType},
    ports::{job_fetcher::JobFetcher, publisher::Publisher},
};

pub struct SchedulerService {
    repo: Arc<dyn JobFetcher>,
    publisher: Arc<dyn Publisher>,
}

impl SchedulerService {
    pub fn new(repo: Arc<dyn JobFetcher>, publisher: Arc<dyn Publisher>) -> Self {
        Self { repo, publisher }
    }

    pub async fn run(&self) -> Result<(), JobSchedulerError> {
        debug!("fetching jobs from db");
        let jobs = self.repo.fetch_jobs_batch().await?;

        if jobs.is_empty() {
            debug!("no pending jobs, returning");
            return Ok(());
        }

        let batch_tracker = tokio_util::task::TaskTracker::new();

        debug!("processing {} jobs", jobs.len());
        for job in jobs {
            let publisher = self.publisher.clone();

            batch_tracker.spawn(async move {
                let job = Job::from_database_type(job);

                if job.job_type() == JobType::Unspecified {
                    error!("Encountered unknown job type in job id {}", job.id);
                    return;
                }

                match publisher.publish(job).await {
                    Ok(..) => (),
                    Err(e) => error!("Failed to publish job: {:?}", e),
                }
            });
        }

        batch_tracker.close();

        batch_tracker.wait().await;

        Ok(())
    }
}
