use std::sync::Arc;

use config::PostgresConfig;
use gen_proto_types::job_result::v1::JobResult;

use crate::{
    adapters::outbound::postgres::PostgresAdapter, core::service::evalservice::EvalService,
};
mod adapters;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = PostgresConfig::from_env()?;
    let postgres_adapter = Arc::new(PostgresAdapter::new(&config.postgres_connection_url())?);
    let eval_service = EvalService::new(postgres_adapter);
    let test_result = JobResult {
        id: "L5tboqyp3NGfq-ZYlWEFg".to_string(),
        batch_id: "b".to_string(),
        run_id: "a".to_string(),
        timestamp: None,
        http: None,
        ping: None,
    };

    let _ = eval_service.evaluate_job_result(&test_result);
    Ok(())
}
