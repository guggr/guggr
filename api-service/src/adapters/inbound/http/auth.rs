use std::sync::Arc;

use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, post, web};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::core::{
    domain::{auth_helper::invalidate_token, openapi_helper},
    models::auth::{
        AuthenticatedResponse, LoginRequest, LogoutRequest, TokenRefreshRequest, TokenResponse,
    },
    ports::{service::ServicePort, storage::StoragePort},
};

/// Configures all auth endpoints
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/auth")
        .service(login)
        .service(token_refresh)
        .service(logout);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = LoginRequest,
    operation_id = "auth_login",
    responses(
        (status = 200, description = "Login successful", body = AuthenticatedResponse),
        openapi_helper::ResBadRequest,
        openapi_helper::ResInternalServerError,
    ),
    tag = "auth"
)]
#[post("/login")]
/// Login
pub async fn login(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: web::Json<LoginRequest>,
) -> actix_web::Result<impl Responder> {
    let res = web::block(move || svc.login(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    request_body = TokenRefreshRequest,
    operation_id = "auth_refresh_token",
    responses(
        (status = 200, description = "Renewed access and refresh tokens", body = TokenResponse),
        openapi_helper::ResBadRequest,
        openapi_helper::ResInternalServerError,
    ),
    tag = "auth"
)]
#[post("/token/refresh")]
/// Renew access and refresh tokens
pub async fn token_refresh(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: web::Json<TokenRefreshRequest>,
) -> actix_web::Result<impl Responder> {
    let res = web::block(move || svc.refresh_auth_tokens(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(res))
}

#[utoipa::path(
    request_body = TokenRefreshRequest,
    operation_id = "auth_logout",
    responses(
        (status = 204, description = "Successful logout"),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResInternalServerError,
    ),
    tag = "auth"
)]
#[post("/logout")]
/// logout endpoint
pub async fn logout(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<LogoutRequest>,
) -> actix_web::Result<impl Responder> {
    web::block(move || invalidate_token(api.get_ref(), &body.into_inner().refresh_token))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
