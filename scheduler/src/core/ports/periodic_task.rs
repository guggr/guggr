use async_trait::async_trait;

use crate::core::domain::errors::JobSchedulerError;

#[async_trait]
pub trait PeriodicTask: Send + Sync {
    /// Executes one tick of the periodic task.
    async fn run(&self) -> Result<(), JobSchedulerError>;
}
