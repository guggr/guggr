use async_trait::async_trait;

use crate::core::domain::errors::JobSchedulerError;

#[async_trait]
pub trait TokenCleaner: Send + Sync {
    /// Deletes all expired refresh tokens, returning the number of rows deleted.
    async fn delete_expired_tokens(&self) -> Result<usize, JobSchedulerError>;
}
