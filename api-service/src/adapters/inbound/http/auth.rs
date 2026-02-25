use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, error::ErrorInternalServerError, post, web};
use config::ApiServiceConfig;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::core::{
    domain::{
        auth_helper::{JwtSigner, get_unverified_user_id, verify_password},
        errors::AuthError,
    },
    models::auth::{AuthMetadata, LoginRequest, LogoutRequest, TokenRefreshRequest, TokenResponse},
    ports::storage::StoragePort,
};

/// configures all paths under the subpath `/auth`
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/auth")
        .service(login)
        .service(token_refresh)
        .service(logout);

    cfg.service(scope);
}

// get auth relevant metadata like ip and user agent
pub fn _get_auth_metadata(req: &HttpRequest) -> AuthMetadata {
    let ip = req
        .peer_addr()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();
    let ua = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    AuthMetadata {
        ip_address: ip,
        user_agent: ua.to_string(),
    }
}

#[utoipa::path(
    request_body = LoginRequest,
    operation_id = "auth_login",
    responses(
        (status = 200, description = "Access- and refresh-token", body = TokenResponse),
        (status = 401, description = "Not Authorized"),
        (status = 500, description = "Storage error")
    ),
    tag = "auth"
)]
#[post("/login")]
/// login endpoint
pub async fn login(
    api: web::Data<Arc<dyn StoragePort>>,
    config: web::Data<ApiServiceConfig>,
    body: web::Json<LoginRequest>,
) -> actix_web::Result<impl Responder> {
    let login_req = body.into_inner();
    let token_response = web::block(move || {
        let Ok(user) = api.auth().get_user_by_email(&login_req.email) else {
            return Err(AuthError::Unauthorized);
        };
        let ok = verify_password(&login_req.password, &user.password).unwrap_or(false);
        if !ok {
            return Err(AuthError::Unauthorized);
        }
        let signer = JwtSigner::new(&config.auth_secret(), &user.jwt_secret);

        signer.create_token(&user.id, config.get_ref(), api.get_ref())
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(token_response))
}

#[utoipa::path(
    request_body = TokenRefreshRequest,
    operation_id = "auth_refresh_token",
    responses(
        (status = 200, description = "Access- and refresh-token", body = TokenResponse),
        (status = 401, description = "Not Authorized"),
        (status = 500, description = "Storage error")
    ),
    tag = "auth"
)]
#[post("/token/refresh")]
/// token refresh endpoint
pub async fn token_refresh(
    api: web::Data<Arc<dyn StoragePort>>,
    config: web::Data<ApiServiceConfig>,
    body: web::Json<TokenRefreshRequest>,
) -> actix_web::Result<impl Responder> {
    let token_response = web::block(move || {
        let old_token = body.into_inner().refresh_token;
        let unverified_user = get_unverified_user_id(&old_token)?;

        let user = api.auth().get_user_jwt_secrets(&unverified_user)?;
        let signer = JwtSigner::new(&config.auth_secret(), &user.jwt_secret);

        signer.refresh_token(config.get_ref(), api.get_ref(), &old_token)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(token_response))
}

#[utoipa::path(
    request_body = TokenRefreshRequest,
    operation_id = "auth_logout",
    responses(
        (status = 204, description = "Successful logout"),
        (status = 500, description = "Storage error")
    ),
    tag = "auth"
)]
#[post("/logout")]
/// logout endpoint
pub async fn logout(
    api: web::Data<Arc<dyn StoragePort>>,
    config: web::Data<ApiServiceConfig>,
    body: web::Json<LogoutRequest>,
) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let old_token = body.into_inner().refresh_token;
        let unverified_user = get_unverified_user_id(&old_token)?;

        let user = api.auth().get_user_jwt_secrets(&unverified_user)?;
        let signer = JwtSigner::new(&config.auth_secret(), &user.jwt_secret);
        signer.invalidate_token(api.get_ref(), &old_token)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
