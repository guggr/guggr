use std::sync::Arc;

use actix_web::{HttpServer, web::Data};
use anyhow::Result;
use api_service::{
    adapters::{inbound::http::app, outgoing::postgres::PostgresAdapter},
    telemetry::init_tracing,
};
use config::PostgresConfig;
use tracing::debug;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = PostgresConfig::from_env()?;
    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres = Arc::from(PostgresAdapter::new(&config.postgres_connection_url())?);
    //example_usage(postgres).await?;
    HttpServer::new(move || app(Data::new(postgres.clone())))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await?;

    Ok(())
}
