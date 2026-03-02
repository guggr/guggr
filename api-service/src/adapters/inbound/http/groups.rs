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
            group::{CreateGroup, DisplayGroup},
        },
        ports::service::ServicePort,
    },
};

/// Configures all groups endpoints
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/groups")
        .wrap(Auth)
        .service(create)
        .service(list);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateGroup,
    operation_id = "create_group",
    responses(
        (status = 200, description = "Group successfully created", body = DisplayGroup),
        openapi_helper::ResBadRequest,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "groups"
)]
#[post("")]
/// Create group
pub async fn create(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: Json<CreateGroup>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let user = web::block(move || svc.create_group(auth_user, body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    operation_id = "list_groups",
    responses(
        (status = 200, description = "User", body = [DisplayGroup]),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "groups"
)]
#[get("")]
/// List groups visible by the logged in user
pub async fn list(
    svc: web::Data<Arc<dyn ServicePort>>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;

    let groups = web::block(move || svc.list_groups_by_user(auth_user))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(groups))
}
