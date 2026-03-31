use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, error::ErrorInternalServerError, get, post,
    put, web,
};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::{errors::DomainError, openapi_helper},
        models::{
            auth::UserId,
            group::{CreateGroup, DisplayGroup, UpdateRequestGroup},
            pagination::{PaginatedResponse, PaginationQuery},
        },
        ports::service::ServicePort,
    },
};

/// Configures all groups endpoints
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/groups")
        .wrap(Auth)
        .service(create)
        .service(list)
        .service(get)
        .service(update);

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
    params(PaginationQuery),
    operation_id = "list_groups",
    responses(
        (status = 200, description = "User", body = PaginatedResponse<DisplayGroup>),
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
    query: web::Query<PaginationQuery>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let params = query.into_inner();

    let groups = web::block(move || svc.list_groups_by_user(&params, auth_user))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(groups))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group ID")
    ),
    operation_id = "get_group",
    responses(
        (status = 200, description = "Group", body = DisplayGroup),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "groups"
)]
#[get("/{id}")]
/// Get group by ID
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

    let group = web::block(move || svc.get_group(auth_user, &path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(group))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group ID")
    ),
    request_body = UpdateRequestGroup,
    operation_id = "update_group",
    responses(
        (status = 200, description = "Group successfully updated", body = DisplayGroup),
        openapi_helper::ResBadRequest,
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "groups"
)]
#[put("/{id}")]
/// Update Group
pub async fn update(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: web::Json<UpdateRequestGroup>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let group =
        web::block(move || svc.update_group(auth_user, &path.into_inner(), body.into_inner()))
            .await
            .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(group))
}
