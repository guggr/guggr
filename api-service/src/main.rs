use anyhow::Result;
use api_service::{adapters::outgoing::postgres::PostgresAdapter, telemetry::init_tracing};
use config::PostgresConfig;
use tracing::debug;

fn main() -> Result<()> {
    init_tracing();
    let config = PostgresConfig::from_env()?;
    debug!("initializing postgres adapter and running pending migrations on the database");
    let _postgres = PostgresAdapter::new(&config.postgres_connection_url())?;

    Ok(())
}
