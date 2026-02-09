use async_trait::async_trait;
use database_client::models::Job;

use crate::core::domain::errors::JobRepositoryError;

#[async_trait]
pub trait JobFetcher: Send {
    async fn fetch_jobs_batch(&self) -> Result<Vec<Job>, JobRepositoryError>;
}
