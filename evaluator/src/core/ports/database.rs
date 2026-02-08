use async_trait::async_trait;
use gen_proto_types::job_result::v1::JobResult;

use crate::core::service::evalservice::EvalServiceError;

#[async_trait]
pub trait DatabasePort {
    async fn notification_enabled(&self, job_id: &str) -> anyhow::Result<bool, EvalServiceError>;
    async fn write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> anyhow::Result<(), EvalServiceError>;
}
