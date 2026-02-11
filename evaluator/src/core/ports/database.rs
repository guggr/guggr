use async_trait::async_trait;
use gen_proto_types::job_result::v1::JobResult;

use crate::core::domain::errors::JobEvaluatorError;

#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn notification_enabled(&self, job_id: &str) -> Result<bool, JobEvaluatorError>;
    async fn write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> Result<(), JobEvaluatorError>;
}
