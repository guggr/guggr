use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, error::ErrorInternalServerError, get, post,
    web,
};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::{errors::DomainError, openapi_helper},
        models::{
            auth::UserId,
            user::{CreateUser, DisplayUser},
        },
        ports::service::ServicePort,
    },
};

/// Configures all users endpoints
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/users")
        .service(create)
        .wrap(Auth)
        .service(get);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateUser,
    operation_id = "create_user",
    responses(
        (status = 200, description = "User successfully created", body = DisplayUser),
        openapi_helper::ResBadRequest,
        openapi_helper::ResInternalServerError,
    ),
    tag = "users"
)]
#[post("")]
/// Create user
pub async fn create(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: Json<CreateUser>,
    _req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || svc.create_user(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "User ID")
    ),
    operation_id = "get_user",
    responses(
        (status = 200, description = "User", body = DisplayUser),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "users"
)]
#[get("/{id}")]
/// Get user by ID
pub async fn get(
    svc: web::Data<Arc<dyn ServicePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;

    let user = web::block(move || svc.get_user(auth_user, &path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(user))
}
