use std::sync::Arc;

use gen_proto_types::job_result::v1::JobResult;
use thiserror::Error;

use crate::core::ports::database::DatabasePort;

#[derive(Debug, Error)]
pub enum EvalServiceError {
    #[error("unknown job id supplied")]
    UnknownJobId,
    #[error("database error: {0}")]
    Db(#[from] diesel::result::Error),
    #[error("database pool error: {0}")]
    DbPool(#[from] diesel::r2d2::PoolError),
}

pub struct EvalService {
    postgres_adapter: Arc<dyn DatabasePort + Send + Sync>,
}

impl Clone for EvalService {
    fn clone(&self) -> Self {
        Self {
            postgres_adapter: Arc::clone(&self.postgres_adapter),
        }
    }
}

impl EvalService {
    pub fn new(postgres_adapter: Arc<dyn DatabasePort + Send + Sync>) -> Self {
        EvalService { postgres_adapter }
    }

    pub async fn evaluate_job_result(
        &self,
        job_result: &JobResult,
    ) -> anyhow::Result<(), EvalServiceError> {
        let notify = self
            .postgres_adapter
            .notification_enabled(&job_result.id)
            .await?;
        if notify {
            // dispatch notifier
            // maybe set notify to false on error?
        }

        self.postgres_adapter
            .write_job_result(job_result, notify)
            .await?;
        Ok(())
    }
}
