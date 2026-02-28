pub mod adapters;
pub mod core;
pub mod telemetry;

use std::sync::Arc;

use actix_web::{App, dev::ServiceFactory, web::Data};
use config::ApiServiceConfig;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    adapters::inbound::http,
    core::{
        domain::openapi_helper::{ApiDoc, json_error_handler},
        ports::storage::StoragePort,
    },
};

/// Initializes and returns configured Actix [`App`] and corresponding
/// [`utoipa::OpenApi`] specification.
///
/// If `api` and `dconfig` options are set, they will be propagated as app data.
/// If `enable_openapi_endpoints` is set, the Swagger UI and the OpenAPI
/// specification will be available at `/api/swagger-ui/`.
#[allow(clippy::type_complexity)]
pub fn init_app(
    api: Option<Data<Arc<dyn StoragePort>>>,
    dconfig: Option<Data<ApiServiceConfig>>,
    enable_openapi_endpoints: Option<bool>,
) -> (
    App<
        impl ServiceFactory<
            actix_web::dev::ServiceRequest,
            Config = (),
            Response = actix_web::dev::ServiceResponse<
                tracing_actix_web::StreamSpan<actix_web::body::BoxBody>,
            >,
            Error = actix_web::Error,
            InitError = (),
        >,
    >,
    utoipa::openapi::OpenApi,
) {
    let mut app = App::new()
        .wrap(TracingLogger::default())
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .app_data(garde_actix_web::web::JsonConfig::default().error_handler(json_error_handler))
        .service(
            utoipa_actix_web::scope("/api/v1")
                .configure(http::groups::configure)
                .configure(http::auth::configure),
        );

    if let Some(api) = api {
        app = app.app_data(api.clone());
    }
    if let Some(dconfig) = dconfig {
        app = app.app_data(dconfig.clone());
    }

    if enable_openapi_endpoints.unwrap_or(false) {
        app = app.openapi_service(|api| {
            SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", api)
        })
    }

    app.split_for_parts()
}
