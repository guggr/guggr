use std::sync::Arc;

use async_trait::async_trait;
use tracing::debug;

use crate::core::{
    domain::errors::JobSchedulerError,
    ports::{periodic_task::PeriodicTask, token_cleaner::TokenCleaner},
};

/// Service logic for cleaning up expired refresh tokens.
pub struct CleanupService {
    cleaner: Arc<dyn TokenCleaner>,
}

impl CleanupService {
    pub fn new(cleaner: Arc<dyn TokenCleaner>) -> Self {
        Self { cleaner }
    }
}

#[async_trait]
impl PeriodicTask for CleanupService {
    async fn run(&self) -> Result<(), JobSchedulerError> {
        debug!("deleting expired refresh tokens");
        let count = self.cleaner.delete_expired_tokens().await?;
        debug!("deleted {} expired refresh tokens", count);
        Ok(())
    }
}
