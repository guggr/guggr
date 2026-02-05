use anyhow::Result;
use gen_proto_types::job::v1::Job;

pub trait Publisher: Send + Sync {
    async fn publish(&self, job: Job) -> Result<()>;
}
