use async_trait::async_trait;

use crate::core::domain::{errors::JobSchedulerError, type_mapper::DatabaseJob};

#[async_trait]
pub trait JobFetcher: Send + Sync {
    /// Fetches job batches via the specific adapter implementation.
    async fn fetch_jobs_batch(&self) -> Result<Vec<DatabaseJob>, JobSchedulerError>;
}
