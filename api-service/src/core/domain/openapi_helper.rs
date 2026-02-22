use utoipa::{
    Modify,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
#[derive(utoipa::OpenApi)]
#[openapi(
    info(description = "guggr api definition"),
    modifiers(&JwtSecurityScheme),
)]
pub struct ApiDoc;

pub struct JwtSecurityScheme;

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
