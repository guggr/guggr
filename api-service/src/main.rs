use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use anyhow::Result;
use api_service::{
    adapters::{
        inbound::http::{self, auth, groups, job, role, users},
        outgoing::postgres::PostgresAdapter,
    },
    core::{domain::openapi_helper::ApiDoc, ports::storage::StoragePort},
    telemetry::init_tracing,
};
use config::{ApiServiceConfig, PostgresConfig};
use tracing::debug;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_actix_web::{self, AppExt};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> Result<()> {
    init_tracing();
    let postgres_config = PostgresConfig::from_env()?;
    let config = ApiServiceConfig::from_env()?;
    debug!("initializing postgres adapter and running pending migrations on the database");
    let postgres: Arc<dyn StoragePort> =
        Arc::from(PostgresAdapter::new(&postgres_config.connection_url())?);
    let api = Data::new(postgres.clone());
    let dconfig = Data::new(config.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(api.clone())
            .app_data(dconfig.clone())
            .service(
                utoipa_actix_web::scope("/api/v1")
                    .configure(groups::configure)
                    .configure(users::configure)
                    .configure(job::configure)
                    .configure(role::configure)
                    .configure(auth::configure)
                    .configure(http::configure),
            )
            .openapi_service(|api| {
                SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", api)
            })
            .into_app()
    })
    .bind(config.bind_address())?
    .run()
    .await?;

    Ok(())
}
