use std::sync::Arc;

use actix_web::{HttpResponse, Responder, delete, get, patch, post, web};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::{ErrorBody, map_storage_error},
    core::{
        models::user::{CreateUser, DisplayUser, UpdateUser},
        ports::storage::StoragePort,
    },
};

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/users")
        .service(create)
        .service(list)
        .service(get)
        .service(delete)
        .service(update);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateUser,
    operation_id = "create_user",
    responses(
        (status = 200, description = "Created user", body = DisplayUser),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "users"
)]
#[post("")]
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<CreateUser>,
) -> impl Responder {
    match api.user().create(body.into_inner()).await {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
    operation_id = "list_user",
    responses(
        (status = 200, description = "List users", body = [DisplayUser]),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "users"
)]
#[get("")]
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> impl Responder {
    match api.user().list(5).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
        operation_id = "get_user",
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User", body = DisplayUser),
        (status = 404, description = "User Not Found", body = ErrorBody),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "users"
)]
#[get("/{id}")]
pub async fn get(api: web::Data<Arc<dyn StoragePort>>, path: web::Path<String>) -> impl Responder {
    match api.user().get_by_id(&path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().json("not found"),
        Err(e) => map_storage_error(e),
    }
}

#[utoipa::path(
        operation_id = "update_user",

    params(
        ("id" = String, Path, description = "User id")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Patched user", body = DisplayUser),
        (status = 404, description = "Empty Body", body = ErrorBody),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "users"
)]
#[patch("/{id}")]
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: web::Json<UpdateUser>,
) -> impl Responder {
    match api
        .user()
        .update(&path.into_inner(), body.into_inner())
        .await
    {
        Ok(r) => HttpResponse::Ok().json(r),
        Err(e) => map_storage_error(e),
    }
}
#[utoipa::path(
        operation_id = "delete_user",

    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 500, description = "Storage error", body = ErrorBody)
    ),
    tag = "users"
)]
#[delete("/{id}")]
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    match api.user().delete(&id).await {
        Ok(()) => HttpResponse::NoContent().finish(),
        Err(e) => map_storage_error(e),
    }
}
