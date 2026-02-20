pub mod groups;

use std::sync::Arc;

use actix_web::{App, HttpResponse, Responder, get, http::header, web};
use tracing::debug;
use utoipa::ToSchema;
use utoipa_actix_web::{self, AppExt};
use utoipa_swagger_ui::SwaggerUi;

use crate::core::{domain::errors::StorageError, ports::storage::StoragePort};

pub fn app(
    api: web::Data<Arc<dyn StoragePort>>,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    debug!("creating new app");
    App::new()
        .into_utoipa_app()
        .app_data(api)
        .service(
            utoipa_actix_web::scope("/api/v1")
                .service(ping)
                .service(
                    utoipa_actix_web::scope("/groups")
                        .service(groups::create)
                        .service(groups::list)
                        .service(groups::get)
                        .service(groups::delete)
                        .service(groups::update),
                )
                .service(openapi_json_redirect)
                .service(swagger_ui_redirect),
        )
        .openapi_service(|api| {
            SwaggerUi::new("/api/swagger-ui/{_:.*}").url("/api/openapi.json", api)
        })
        .into_app()
}

#[utoipa::path(
    responses(
        (status = 200, description = "pong"),
    ),
)]
#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[utoipa::path(
    responses(
        (status = 307, description = "Temporary redirect to /api/openapi.json")
    ),
    tag = "docs"
)]
#[get("/openapi.json")]
async fn openapi_json_redirect() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, "/api/openapi.json"))
        .finish()
}

#[utoipa::path(
    responses(
        (status = 307, description = "Temporary redirect to /api/swagger-ui/")
    ),
    tag = "docs"
)]
#[get("/swagger-ui/{_:.*}")]
async fn swagger_ui_redirect() -> impl Responder {
    HttpResponse::TemporaryRedirect()
        .insert_header((header::LOCATION, "/api/swagger-ui/"))
        .finish()
}

fn map_storage_error(err: StorageError) -> HttpResponse {
    match err {
        StorageError::Internal(_) => HttpResponse::InternalServerError()
            .json(err_body("unexpected", "Something went wrong".to_string())),
        StorageError::Unavailable(_) => HttpResponse::InternalServerError()
            .json(err_body("unexpected", "Something went wrong".to_string())),
    }
}

#[derive(serde::Serialize, ToSchema)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

fn err_body(code: &'static str, message: String) -> ErrorBody {
    ErrorBody { code, message }
}
