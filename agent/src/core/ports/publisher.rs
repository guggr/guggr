use async_trait::async_trait;
use gen_proto_types::job_result::v1::JobResult;

#[async_trait]
pub trait PublisherPort {
    async fn publish_result(&self, job_result: &JobResult) -> anyhow::Result<()>;
}
