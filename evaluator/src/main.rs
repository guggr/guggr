use std::sync::Arc;

use config::PostgresConfig;

use crate::{
    adapters::outbound::postgres::PostgresAdapter, core::service::evalservice::EvalService,
};
mod adapters;
mod core;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_filename(".env.sample").ok();

    let config = PostgresConfig::from_env()?;
    let postgres_adapter = Arc::new(PostgresAdapter::new(&config.postgres_connection_url())?);
    Ok(())
}
