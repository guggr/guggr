use std::sync::Arc;

use gen_proto_types::job::v1::{Job, JobType};
use nanoid::nanoid;
use tracing::{debug, error};

use crate::core::{
    domain::{errors::JobSchedulerError, type_mapper::JobFromDatabaseJobResult},
    ports::{job_fetcher::JobFetcher, publisher::Publisher},
};

/// Service logic for fetching jobs and publishing them.
pub struct SchedulerService {
    repo: Arc<dyn JobFetcher>,
    publisher: Arc<dyn Publisher>,
}

impl SchedulerService {
    /// Creates a new [`SchedulerService`] instance using a given fetcher and
    /// publisher.
    pub fn new(repo: Arc<dyn JobFetcher>, publisher: Arc<dyn Publisher>) -> Self {
        Self { repo, publisher }
    }

    /// Runs the business logic for fetching jobs from the database and pushing
    /// them to the database.
    ///
    /// # Errors
    /// Returns an error if fetching the jobs from the database failed
    ///
    /// # Error Handling
    /// Logs an error if
    /// - a [`JobType::Unspecified`] is encountered
    /// - publishing the job the job failed
    ///
    /// This does not stop the run from continuing, further jobs are published
    /// even if an error was logged.
    pub async fn run(&self) -> Result<(), JobSchedulerError> {
        debug!("fetching jobs from db");
        let jobs = self.repo.fetch_jobs_batch().await?;

        if jobs.is_empty() {
            debug!("no pending jobs, returning");
            return Ok(());
        }

        let batch_tracker = tokio_util::task::TaskTracker::new();
        let batch_id = nanoid!();

        debug!("processing {} jobs with batch id {}", jobs.len(), batch_id);
        for job in jobs {
            let publisher = self.publisher.clone();
            let batch_id = batch_id.clone();

            batch_tracker.spawn(async move {
                let job = Job::from_database_type(job, batch_id);

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
