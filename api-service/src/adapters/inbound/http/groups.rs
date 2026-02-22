use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::{ErrorBody, map_storage_error, middleware::auth::Auth},
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
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[post("")]
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: Json<CreateGroup>,
) -> impl Responder {
    match api.group().create(body.into_inner()).await {
        Ok(r) => HttpResponse::NoContent().json(r),
        Err(e) => map_storage_error(&e),
    }
}

#[utoipa::path(
        operation_id = "list_group",
    responses(
        (status = 200, description = "List groups", body = [DisplayGroup]),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[get("")]
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> impl Responder {
    match api.group().list(5).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => map_storage_error(&e),
    }
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    operation_id = "get_group",
    responses(
        (status = 200, description = "Group", body = DisplayGroup),
        (status = 404, description = "Group Not Found", body = ErrorBody),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[get("/{id}")]
pub async fn get(api: web::Data<Arc<dyn StoragePort>>, path: web::Path<String>) -> impl Responder {
    match api.group().get_by_id(&path.into_inner()).await {
        Ok(Some(group)) => HttpResponse::Ok().json(group),
        Ok(None) => HttpResponse::NotFound().json("not found"),
        Err(e) => map_storage_error(&e),
    }
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
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[patch("/{id}")]
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: Json<UpdateGroup>,
) -> impl Responder {
    match api
        .group()
        .update(&path.into_inner(), body.into_inner())
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => map_storage_error(&e),
    }
}
#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    operation_id = "delete_group",
    responses(
        (status = 204, description = "Deleted"),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    security(("bearerAuth" = [])),
    tag = "groups"
)]
#[delete("/{id}")]
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match api.group().delete(&id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => map_storage_error(&e),
    }
}
