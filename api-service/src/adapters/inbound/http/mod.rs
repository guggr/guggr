pub mod auth;
pub mod groups;
pub mod job;
pub mod middleware;
pub mod users;
use actix_web::{
    HttpResponse, Responder, ResponseError, get,
    http::{StatusCode, header},
};
use utoipa_actix_web::{self, service_config::ServiceConfig};

use crate::core::domain::errors::{AuthError, StorageError};

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

impl ResponseError for StorageError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Internal(_) | Self::Unavailable(_) | Self::TimestampConversion => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::Internal(_) | Self::Unavailable(_) | Self::TimestampConversion => {
                HttpResponse::InternalServerError().finish()
            }
        }
    }
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::JwtError(_) => HttpResponse::InternalServerError().finish(),
            _ => HttpResponse::Unauthorized().finish(),
        }
    }
}
