use utoipa::{
    Modify,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
#[derive(utoipa::OpenApi)]
#[openapi(
    modifiers(&JwtSecurityScheme),
)]
pub struct ApiDoc;

pub struct JwtSecurityScheme;

/// Adds the security scheme `bearerAuth` to the openapi spec
impl Modify for JwtSecurityScheme {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
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

#[derive(utoipa::IntoResponses)]
/// Generic Responses used for list, get, delete
pub enum GenericResponses {
    #[response(status = 401)]
    Unauthorized(#[to_response] Unauthorized),

    #[response(status = 404)]
    NotFound(#[to_response] NotFound),

    #[response(status = 500)]
    InternalServerError(#[to_response] InternalServerError),
}

#[derive(utoipa::IntoResponses)]
/// Generic Responses used for create, update
pub enum GenericResponsesCU {
    #[response(status = 400)]
    Validation(#[to_response] Validation),

    #[response(status = 401)]
    Unauthorized(#[to_response] Unauthorized),

    #[response(status = 404)]
    NotFound(#[to_response] NotFound),

    #[response(status = 500)]
    InternalServerError(#[to_response] InternalServerError),
}

#[derive(utoipa::IntoResponses)]
/// Generic Responses used for auth endpoints
pub enum GenericResponsesAuth {
    #[response(status = 400)]
    Validation(#[to_response] Validation),

    #[response(status = 401)]
    Unauthorized(#[to_response] Unauthorized),

    #[response(status = 500)]
    InternalServerError(#[to_response] InternalServerError),
}

#[derive(utoipa::ToResponse)]
#[response(description = "Internal Server Error")]
pub struct InternalServerError;

#[derive(utoipa::ToResponse)]
#[response(description = "Not Found")]
pub struct NotFound;

#[derive(utoipa::ToResponse)]
#[response(description = "User is not Authorized / Authenticated")]
pub struct Unauthorized;

#[derive(utoipa::ToResponse)]
#[response(description = "Validation / Deserializing failed")]
pub struct Validation;
