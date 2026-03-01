use actix_web::{
    HttpRequest, HttpResponse, ResponseError, error::InternalError, http::header::ContentType,
};
use serde::Serialize;
use utoipa::{
    Modify, ToSchema,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
#[derive(utoipa::OpenApi)]
#[openapi(
    modifiers(&JwtSecurityScheme),
    components(schemas(BadRequestErrorBody))
)]
pub struct ApiDoc;

pub struct JwtSecurityScheme;

/// Adds the security scheme `bearerAuth` to the openapi spec
impl Modify for JwtSecurityScheme {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
/// Bad request errors
pub struct BadRequestErrorBody {
    pub message: String,
}

/// error handler that converts errors into validation errors
pub fn json_error_handler(
    err: garde_actix_web::error::Error,
    _req: &HttpRequest,
) -> actix_web::Error {
    let status = err.status_code();

    let resp = HttpResponse::build(status)
        .insert_header(ContentType::json())
        .json(BadRequestErrorBody {
            message: err.to_string(),
        });

    InternalError::from_response(err, resp).into()
}

#[derive(utoipa::ToResponse)]
#[response(description = "Bad Request")]
pub struct BadRequest(pub BadRequestErrorBody);

#[derive(utoipa::ToResponse)]
#[response(description = "Unauthorized")]
pub struct Unauthorized;

#[derive(utoipa::ToResponse)]
#[response(description = "Not Found")]
pub struct NotFound;

#[derive(utoipa::ToResponse)]
#[response(description = "Internal Server Error")]
pub struct InternalServerError;

#[derive(utoipa::IntoResponses)]
/// Response: 400 Bad Request
pub enum ResBadRequest {
    #[response(status = 400)]
    BadRequest(#[to_response] BadRequest),
}

#[derive(utoipa::IntoResponses)]
/// Response: 401 Unauthorized
pub enum ResUnauthorized {
    #[response(status = 401)]
    Unauthorized(#[to_response] Unauthorized),
}

#[derive(utoipa::IntoResponses)]
/// Response: 404 Not Found
pub enum ResNotFound {
    #[response(status = 404)]
    NotFound(#[to_response] NotFound),
}

#[derive(utoipa::IntoResponses)]
/// Response: 500 Internal Server Error
pub enum ResInternalServerError {
    #[response(status = 500)]
    InternalServerError(#[to_response] InternalServerError),
}
