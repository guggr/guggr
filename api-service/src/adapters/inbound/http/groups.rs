use std::sync::Arc;

use actix_web::{HttpResponse, Responder, web};
use database_client::models::Group;
use nanoid::nanoid;

use crate::{adapters::inbound::http::map_storage_error, core::ports::storage::StoragePort};

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

pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> impl Responder {
    match api.group().list(5).await {
        Ok(groups) => HttpResponse::Ok().json(groups),
        Err(e) => map_storage_error(e),
    }
}

pub async fn get(api: web::Data<Arc<dyn StoragePort>>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    match api.group().get_by_id(&id).await {
        Ok(group) => HttpResponse::Ok().json(group),
        Err(e) => map_storage_error(e),
    }
}

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
