use anyhow::Result;
use api_service::{
    adapters::outgoing::postgres::PostgresAdapter, example_usage, telemetry::init_tracing,
};
use config::PostgresConfig;
use tracing::debug;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = PostgresConfig::from_env()?;
    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres = PostgresAdapter::new(&config.postgres_connection_url())?;

    example_usage(postgres).await?;

    Ok(())
}
