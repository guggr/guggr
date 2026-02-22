use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, post, web};
use compact_jwt::JwsEs256Signer;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::{ErrorBody, map_auth_error},
    core::{
        domain::auth_helper::{create_token, refresh_token, verify_password},
        models::auth::{
            AuthMetadata, LoginRequest, TokenRefreshRequest, TokenResponse,
        },
        ports::storage::StoragePort,
    },
};
// TODO move to configuration
const TTL: i64 = 15 * 60;
const TTL_REFRESH: i64 = 60 * 60 * 4;

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/auth")
        .service(login)
        .service(token_refresh);

    cfg.service(scope);
}

// get auth relevant metadata like ip and user agent
pub fn get_auth_metadata(req: &HttpRequest) -> AuthMetadata {
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
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    api: web::Data<Arc<dyn StoragePort>>,
    signer: web::Data<JwsEs256Signer>,
    req: HttpRequest,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    let meta = get_auth_metadata(&req);
    drop(req);
    let login_req = body.into_inner();
    let Ok(user) = api.auth().get_user_by_email(&login_req.email).await else {
        return HttpResponse::Unauthorized().finish();
    };
    let ok = verify_password(&login_req.password, &user.password).unwrap_or(false);
    if !ok {
        return HttpResponse::Unauthorized().finish();
    }

    create_token(
        signer.get_ref(),
        api.get_ref(),
        meta,
        &user.id,
        TTL,
        TTL_REFRESH,
    )
    .await
    .map_or_else(
        |err| map_auth_error(&err),
        |token_response| HttpResponse::Ok().json(token_response),
    )
}

#[utoipa::path(
    request_body = TokenRefreshRequest,
    operation_id = "auth_refresh_token",
    responses(
        (status = 200, description = "Access- and refresh-token", body = TokenResponse),
        (status = 401, description = "Not Authorized"),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "auth"
)]
#[post("/token/refresh")]
pub async fn token_refresh(
    api: web::Data<Arc<dyn StoragePort>>,
    signer: web::Data<JwsEs256Signer>,
    req: HttpRequest,
    body: web::Json<TokenRefreshRequest>,
) -> impl Responder {
    let meta = get_auth_metadata(&req);
    refresh_token(
        signer.get_ref(),
        api.get_ref(),
        meta,
        &body.into_inner().refresh_token,
        TTL,
        TTL_REFRESH,
    )
    .await
    .map_or_else(
        |err| map_auth_error(&err),
        |token_response| HttpResponse::Ok().json(token_response),
    )
}
