use async_trait::async_trait;

use crate::core::domain::{errors::JobRepositoryError, models::Job};

#[async_trait]
pub trait JobFetcher: Send + Sync {
    /// Invokes the adapter-specific implementation to fetch jobs from the
    /// chosen repository.
    async fn fetch_jobs_batch(&self) -> Result<Vec<Job>, JobRepositoryError>;
}
