use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, post, put, web};
use database_client::models::Group;
use nanoid::nanoid;

use crate::{
    adapters::inbound::http::{ErrorBody, map_storage_error},
    core::ports::storage::StoragePort,
};

#[utoipa::path(
    request_body = Group,
    responses(
        (status = 201, description = "Created group", body = Group),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "groups"
)]
#[post("")]
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<Group>,
) -> impl Responder {
    let mut new_group = body.into_inner();
    new_group.id = nanoid!();
    match api.group().create(new_group).await {
        Ok(group) => HttpResponse::Created().json(group),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
    responses(
        (status = 200, description = "List groups", body = [Group]),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "groups"
)]
#[get("")]
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> impl Responder {
    match api.group().list(5).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    responses(
        (status = 200, description = "Group (or null if not found)", body = Option<Group>),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "groups"
)]
#[get("/{id}")]
pub async fn get(api: web::Data<Arc<dyn StoragePort>>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match api.group().get_by_id(&id).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    request_body = Group,
    responses(
        (status = 200, description = "Updated group", body = Group),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "groups"
)]
#[put("/{id}")]
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: web::Json<Group>,
) -> impl Responder {
    let _id = path.into_inner();
    match api.group().update(body.into_inner()).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(e) => map_storage_error(e),
    }
}
#[utoipa::path(
    params(
        ("id" = String, Path, description = "Group id")
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
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
        Err(e) => map_storage_error(e),
    }
}
