pub mod auth;
pub mod groups;
pub mod middleware;
pub mod users;
use actix_web::{HttpResponse, Responder, get, http::header};
use utoipa::ToSchema;
use utoipa_actix_web::{self, service_config::ServiceConfig};

use crate::core::domain::errors::StorageError;

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("")
        .service(ping)
        .service(openapi_json_redirect)
        .service(swagger_ui_redirect);

    cfg.service(scope);
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

fn map_storage_error(err: &StorageError) -> HttpResponse {
    match err {
        StorageError::Internal(_)
        | StorageError::Unavailable(_)
        | StorageError::TimestampConversion => HttpResponse::InternalServerError()
            .json(err_body("unexpected", "Something went wrong".to_string())),
        StorageError::NotFound => {
            HttpResponse::NotFound().json(err_body("not found", "No record found".to_string()))
        }
    }
}

#[derive(serde::Serialize, ToSchema)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

const fn err_body(code: &'static str, message: String) -> ErrorBody {
    ErrorBody { code, message }
}
