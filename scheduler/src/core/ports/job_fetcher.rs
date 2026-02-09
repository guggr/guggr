use async_trait::async_trait;

use crate::core::domain::{errors::JobSchedulerError, type_mapper::DatabaseJobResult};

#[async_trait]
pub trait JobFetcher: Send + Sync {
    async fn fetch_jobs_batch(&self) -> Result<Vec<DatabaseJobResult>, JobSchedulerError>;
}
