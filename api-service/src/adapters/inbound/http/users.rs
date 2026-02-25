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
        models::user::{CreateUser, DisplayUser, UpdateUser},
        ports::storage::StoragePort,
    },
};

/// configures all paths under the subpath `/users`
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/users")
        .wrap(Auth)
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
        GenericResponsesCU
    ),
    security(("bearerAuth" = [])),
    tag = "users"
)]
#[post("")]
/// create endpoint for users
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: Json<CreateUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || api.user().create(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(user))
}

#[utoipa::path(
    operation_id = "list_user",
    responses(
        (status = 200, description = "List users", body = [DisplayUser]),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "users"
)]
#[get("")]
/// list endpoint for users
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> actix_web::Result<impl Responder> {
    let users = web::block(move || api.user().list(5))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(users))
}

#[utoipa::path(
        operation_id = "get_user",
    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 200, description = "User", body = DisplayUser),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "users"
)]
#[get("/{id}")]
/// get endpoint for users
pub async fn get(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    web::block(move || api.user().get_by_id(&path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??
        .map_or_else(
            || Ok(HttpResponse::NotFound().finish()),
            |user| Ok(HttpResponse::Ok().json(user)),
        )
}

#[utoipa::path(
        operation_id = "update_user",

    params(
        ("id" = String, Path, description = "User id")
    ),
    request_body = UpdateUser,
    responses(
        (status = 200, description = "Patched user", body = DisplayUser),
        GenericResponsesCU
    ),
    security(("bearerAuth" = [])),
    tag = "users"
)]
#[patch("/{id}")]
/// update endpoint for users
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: Json<UpdateUser>,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || api.user().update(&path.into_inner(), body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(user))
}
#[utoipa::path(
        operation_id = "delete_user",

    params(
        ("id" = String, Path, description = "User id")
    ),
    responses(
        (status = 204, description = "Deleted"),
        GenericResponses
    ),
    security(("bearerAuth" = [])),
    tag = "users"
)]
#[delete("/{id}")]
/// delete endpoint for users
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();
    web::block(move || api.user().delete(&id))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
