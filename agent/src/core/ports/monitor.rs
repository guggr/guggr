use async_trait::async_trait;
use gen_proto_types::{job::v1::Job, job_result::v1::JobResult};

#[async_trait]
pub trait MonitorPort {
    async fn execute(&self, job: &Job) -> anyhow::Result<JobResult>;
}
