pub mod auth;
pub mod groups;
pub mod middleware;
pub mod users;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};

use crate::core::domain::errors::{AuthError, DomainError};

impl ResponseError for DomainError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Internal(_) | Self::Unavailable(_) | Self::TimestampConversion => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::BadRequest => HttpResponse::BadRequest().finish(),
            Self::NotFound => HttpResponse::NotFound().finish(),
            Self::Unauthorized => HttpResponse::Unauthorized().finish(),
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
