use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        models::group::{CreateGroup, DisplayGroup, UpdateGroup},
        ports::storage::StoragePort,
    },
};

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/groups")
        .wrap(Auth)
        .service(create)
        .service(list)
        .service(get)
        .service(delete)
        .service(update);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateGroup,
    operation_id = "create_group",
    responses(
        (status = 200, description = "Created group", body = DisplayGroup),
        (status = 400, description = "Validation error"), // TODO get ToSchema for ValidationError
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[post("")]
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: Json<CreateGroup>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || api.group().create(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
        operation_id = "list_group",
    responses(
        (status = 200, description = "List groups", body = [DisplayGroup]),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[get("")]
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> actix_web::Result<impl Responder> {
    let groups = web::block(move || api.group().list(5))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(groups))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    operation_id = "get_group",
    responses(
        (status = 200, description = "Group", body = DisplayGroup),
        (status = 404, description = "Group Not Found"),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[get("/{id}")]
pub async fn get(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    web::block(move || api.group().get_by_id(&path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??
        .map_or_else(
            || Ok(HttpResponse::NotFound().finish()),
            |group| Ok(HttpResponse::Ok().json(group)),
        )
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    operation_id = "update_group",
    request_body = UpdateGroup,
    responses(
        (status = 200, description = "Patched group", body = DisplayGroup),
        (status = 400, description = "Validation error"), // TODO get ToSchema for ValidationError
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[patch("/{id}")]
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: Json<UpdateGroup>,
) -> actix_web::Result<impl Responder> {
    let group = web::block(move || api.group().update(&path.into_inner(), body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(group))
}
#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    operation_id = "delete_group",
    responses(
        (status = 204, description = "Deleted"),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[delete("/{id}")]
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();
    web::block(move || api.group().delete(&id))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
