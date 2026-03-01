use std::sync::Arc;

use async_trait::async_trait;
use gen_proto_types::{job::v1::Job, job_types::v1::JobType};
use nanoid::nanoid;
use tracing::{debug, error};

use crate::core::{
    domain::{errors::JobSchedulerError, type_mapper::JobFromDatabaseJob},
    ports::{job_fetcher::JobFetcher, periodic_task::PeriodicTask, publisher::Publisher},
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
    /// - publishing the job that failed
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

#[async_trait]
impl PeriodicTask for SchedulerService {
    async fn run(&self) -> Result<(), JobSchedulerError> {
        SchedulerService::run(self).await
    }
}

#[cfg(test)]
mod tests {

    use std::sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    use async_trait::async_trait;
    use chrono::Duration;
    use database_client::models::Job;

    use super::*;
    use crate::{core::domain::type_mapper::DatabaseJob, telemetry::init_tracing};

    fn mock_job() -> DatabaseJob {
        (
            Job {
                id: "".to_owned(),
                group_id: "".to_owned(),
                job_type_id: "http".to_owned(),
                name: "".to_owned(),
                last_scheduled: None,
                notify_users: false,
                run_every: Duration::new(0, 5000000).unwrap(),
                custom_notification: None,
            },
            None,
            None,
        )
    }

    struct MockFetcher {
        jobs: Mutex<Option<Vec<DatabaseJob>>>,
    }

    impl MockFetcher {
        fn new(jobs: Vec<DatabaseJob>) -> Self {
            Self {
                jobs: Mutex::new(Some(jobs)),
            }
        }
    }

    #[async_trait]
    impl JobFetcher for MockFetcher {
        async fn fetch_jobs_batch(&self) -> Result<Vec<DatabaseJob>, JobSchedulerError> {
            let mut guard = self.jobs.lock().unwrap();

            // .take() returns Some(jobs) the first time, and None every time after
            let jobs = guard.take().ok_or_else(|| {
                JobSchedulerError::Internal(
                    "Mock Jobs already taken/Mock only supports one call".into(),
                )
            })?;

            Ok(jobs)
        }
    }

    struct MockPublisher {
        error_at: Option<usize>,
        counter: AtomicUsize,
        published_counter: AtomicUsize,
    }

    impl MockPublisher {
        fn new(error_at: Option<usize>) -> Self {
            Self {
                error_at,
                counter: AtomicUsize::new(0),
                published_counter: AtomicUsize::new(0),
            }
        }

        fn published_jobs(&self) -> usize {
            self.published_counter.load(Ordering::SeqCst)
        }

        fn failing_jobs(&self) -> usize {
            self.counter.load(Ordering::SeqCst) - self.published_jobs()
        }
    }

    #[async_trait]
    impl Publisher for MockPublisher {
        async fn publish(
            &self,
            _job: gen_proto_types::job::v1::Job,
        ) -> Result<(), JobSchedulerError> {
            let curr_counter = self.counter.load(Ordering::SeqCst);

            if self.error_at.is_some_and(|x| x == curr_counter) {
                self.counter.fetch_add(1, Ordering::SeqCst);
                return Err(JobSchedulerError::Internal(
                    "Triggered requested error".to_owned(),
                ));
            }

            self.counter.fetch_add(1, Ordering::SeqCst);
            self.published_counter.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn init() {
        init_tracing();
    }

    #[tokio::test]
    async fn no_errors() {
        init();

        let mut jobs = Vec::with_capacity(12);

        for _ in 0..15 {
            jobs.push(mock_job());
        }

        let fetcher = Arc::from(MockFetcher::new(jobs));
        let publisher = Arc::from(MockPublisher::new(None));

        let service = SchedulerService::new(fetcher.clone(), publisher.clone());

        assert!(service.run().await.is_ok());
        assert_eq!(publisher.published_jobs(), 15);
        assert_eq!(publisher.failing_jobs(), 0);
    }

    #[tokio::test]
    async fn unspecified_job_type() {
        init();

        let mut unspecified_job_type = mock_job();
        unspecified_job_type.0.job_type_id = "doesnotexist".to_owned();

        let fetcher = Arc::from(MockFetcher::new(vec![
            mock_job(),
            unspecified_job_type,
            mock_job(),
        ]));
        let publisher = Arc::from(MockPublisher::new(None));

        let service = SchedulerService::new(fetcher.clone(), publisher.clone());

        assert!(service.run().await.is_ok());
        assert_eq!(publisher.published_jobs(), 2);
        assert_eq!(publisher.failing_jobs(), 0);
    }

    #[tokio::test]
    async fn no_pending_jobs() {
        init();

        let fetcher = Arc::from(MockFetcher::new(vec![]));
        let publisher = Arc::from(MockPublisher::new(None));

        let service = SchedulerService::new(fetcher.clone(), publisher.clone());

        assert!(service.run().await.is_ok());
        assert_eq!(publisher.published_jobs(), 0);
        assert_eq!(publisher.failing_jobs(), 0);
    }

    #[tokio::test]
    async fn publishing_error() {
        init();

        let fetcher = Arc::from(MockFetcher::new(vec![mock_job(), mock_job(), mock_job()]));
        let publisher = Arc::from(MockPublisher::new(Some(1)));

        let service = SchedulerService::new(fetcher.clone(), publisher.clone());

        assert!(service.run().await.is_ok());
        assert_eq!(publisher.published_jobs(), 2);
        assert_eq!(publisher.failing_jobs(), 1);
    }
}
