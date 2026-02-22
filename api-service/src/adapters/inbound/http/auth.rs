use std::sync::Arc;

use actix_web::{HttpResponse, Responder, post, web};
use compact_jwt::JwsEs256Signer;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::{ErrorBody, map_storage_error},
    core::{
        domain::auth_helper::{create_jwt, verify_password},
        models::auth::{LoginRequest, TokenResponse},
        ports::storage::StoragePort,
    },
};

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/auth").service(login);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = LoginRequest,
    operation_id = "auth_login",
    responses(
        (status = 200, description = "JWT token", body = TokenResponse),
        (status = 401, description = "Not Authorized"),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    api: web::Data<Arc<dyn StoragePort>>,
    signer: web::Data<JwsEs256Signer>,
    body: web::Json<LoginRequest>,
) -> impl Responder {
    // TODO move to configuration
    const TTL: i64 = 15 * 60;
    let req = body.into_inner();
    let user = match api.auth().get_user_by_email(&req.email).await {
        Ok(user) => user,
        Err(e) => return map_storage_error(&e),
    };
    let ok = verify_password(&req.password, &user.password).unwrap_or(false);
    if !ok {
        return HttpResponse::Unauthorized().finish();
    }

    let Ok(jwt) = create_jwt(signer.get_ref(), &user.id, TTL) else {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok().json(TokenResponse { access_token: jwt })
}

// TODO logout
