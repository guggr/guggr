use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::openapi_helper::{GenericResponses, GenericResponsesCU},
        models::role::{CreateRole, DisplayRole, UpdateRole},
        ports::storage::StoragePort,
    },
};

/// configures all paths under the subpath `/roles`
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/roles")
        .wrap(Auth)
        .service(create)
        .service(list)
        .service(get)
        .service(delete)
        .service(update);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateRole,
    operation_id = "create_role",
    responses(
        (status = 200, description = "Created role", body = DisplayRole),
        GenericResponsesCU
    ),
    security(("bearerAuth" = [])),
    tag = "roles"
)]
#[post("")]
/// create endpoint for roles
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: Json<CreateRole>,
) -> actix_web::Result<impl Responder> {
    let role = web::block(move || api.role().create(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(role))
}

#[utoipa::path(
    operation_id = "list_role",
    responses(
        (status = 200, description = "List roles", body = [DisplayRole]),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "roles"
)]
#[get("")]
/// list endpoint for roles
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> actix_web::Result<impl Responder> {
    let roles = web::block(move || api.role().list(5))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(roles))
}

#[utoipa::path(
        operation_id = "get_role",
    params(
        ("id" = String, Path, description = "Role id")
    ),
    responses(
        (status = 200, description = "Role", body = DisplayRole),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "roles"
)]
#[get("/{id}")]
/// get endpoint for roles
pub async fn get(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    web::block(move || api.role().get_by_id(&path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??
        .map_or_else(
            || Ok(HttpResponse::NotFound().finish()),
            |role| Ok(HttpResponse::Ok().json(role)),
        )
}

#[utoipa::path(
        operation_id = "update_role",

    params(
        ("id" = String, Path, description = "Role id")
    ),
    request_body = UpdateRole,
    responses(
        (status = 200, description = "Patched role", body = DisplayRole),
        GenericResponsesCU
    ),
    security(("bearerAuth" = [])),
    tag = "roles"
)]
#[patch("/{id}")]
/// update endpoint for roles
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: Json<UpdateRole>,
) -> actix_web::Result<impl Responder> {
    let role = web::block(move || api.role().update(&path.into_inner(), body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(role))
}
#[utoipa::path(
        operation_id = "delete_role",

    params(
        ("id" = String, Path, description = "Role id")
    ),
    responses(
        (status = 204, description = "Deleted"),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "roles"
)]
#[delete("/{id}")]
/// delete endpoint for roles
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();
    web::block(move || api.role().delete(&id))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
