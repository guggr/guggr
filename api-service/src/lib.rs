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
    adapters::inbound::http::{self, auth, groups, jobs, roles, users},
    core::{domain::openapi_helper::ApiDoc, ports::storage::StoragePort},
};

/// Initializes and returns configured Actix [`App`] and corresponding
/// [`utoipa::OpenApi`] specification.
///
/// If `api` and `dconfig` options are set, they will be propagated as app data.
/// If `enable_swagger_ui` bool is set to `true`, a Swagger UI endpoint will be
/// available at `/api/swagger-ui/`.
#[allow(clippy::type_complexity)]
pub fn init_app_openapi(
    api: Option<Data<Arc<dyn StoragePort>>>,
    dconfig: Option<Data<ApiServiceConfig>>,
    enable_swagger_ui: bool,
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
        .service(
            utoipa_actix_web::scope("/api/v1")
                .configure(groups::configure)
                .configure(users::configure)
                .configure(jobs::configure)
                .configure(roles::configure)
                .configure(auth::configure)
                .configure(http::configure),
        );

    if let Some(api) = api {
        app = app.app_data(api.clone());
    }
    if let Some(dconfig) = dconfig {
        app = app.app_data(dconfig.clone());
    }
    if enable_swagger_ui {
        app = app.openapi_service(|api| {
            SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", api)
        })
    }

    app.split_for_parts()
}
