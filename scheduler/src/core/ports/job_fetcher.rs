use async_trait::async_trait;
use database_client::models::{Job, JobDetailsHttp, JobDetailsPing};

use crate::core::domain::errors::JobRepositoryError;

#[async_trait]
pub trait JobFetcher: Send + Sync {
    async fn fetch_jobs_batch(
        &self,
    ) -> Result<Vec<(Job, Option<JobDetailsHttp>, Option<JobDetailsPing>)>, JobRepositoryError>;
}
