use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use anyhow::Result;
use api_service::{
    adapters::{
        inbound::http::{self, groups, users},
        outgoing::postgres::PostgresAdapter,
    },
    core::ports::storage::StoragePort,
    telemetry::init_tracing,
};
use config::PostgresConfig;
use tracing::debug;
use tracing_actix_web::TracingLogger;
use utoipa_actix_web::{self, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let config = PostgresConfig::from_env()?;
    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres: Arc<dyn StoragePort> = Arc::from(PostgresAdapter::new(&config.connection_url())?);
    let api = Data::new(postgres.clone());
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .into_utoipa_app()
            .app_data(api.clone())
            .service(
                utoipa_actix_web::scope("/api/v1")
                    .configure(groups::configure)
                    .configure(users::configure)
                    .configure(http::configure),
            )
            .openapi_service(|api| {
                SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await?;

    Ok(())
}
