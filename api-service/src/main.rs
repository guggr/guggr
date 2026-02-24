use std::sync::Arc;

use actix_web::{HttpServer, web::Data};
use anyhow::Result;
use api_service::{
    adapters::outgoing::postgres::PostgresAdapter, core::ports::storage::StoragePort,
    init_app_openapi, telemetry::init_tracing,
};
use config::{ApiServiceConfig, PostgresConfig};
use tracing::debug;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let postgres_config = PostgresConfig::from_env()?;

    let config = ApiServiceConfig::from_env()?;
    let bind_address = config.bind_address();

    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres: Arc<dyn StoragePort> =
        Arc::from(PostgresAdapter::new(&postgres_config.connection_url())?);

    HttpServer::new(move || {
        init_app_openapi(
            Some(Data::new(postgres.clone())),
            Some(Data::new(config.clone())),
            true,
        )
        .0
    })
    .bind(bind_address)?
    .run()
    .await?;

    Ok(())
}
