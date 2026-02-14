use async_trait::async_trait;
use gen_proto_types::job::v1::Job;

use crate::core::domain::errors::JobSchedulerError;

#[async_trait]
pub trait Publisher: Send + Sync {
    /// Publishes jobs via the specific adapter implementation.
    async fn publish(&self, job: Job) -> Result<(), JobSchedulerError>;
}
