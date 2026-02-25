use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, delete, error::ErrorInternalServerError,
    get, patch, post, web,
};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::errors::{AuthError, StorageError},
        models::{
            auth::UserId,
            group::{CreateGroup, DisplayGroup, UpdateGroup},
        },
        ports::storage::StoragePort,
    },
};

/// configures all paths under the subpath `/groups`
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
/// create endpoint for groups
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: Json<CreateGroup>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let group = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.group().create(body.into_inner());
        }
        Err(StorageError::Unauthorized)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(group))
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
/// list endpoint for groups
pub async fn list(
    api: web::Data<Arc<dyn StoragePort>>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let groups = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.group().list(None, 5);
        }
        api.group().list(Some(&req_userid.0), 5)
    })
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
/// get endpoint for groups
pub async fn get(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.group().get_by_id(None, &path.into_inner());
        }

        api.group()
            .get_by_id(Some(&req_userid.0), &path.into_inner())
    })
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
/// update endpoint for groups
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    body: Json<UpdateGroup>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let group = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api
                .group()
                .update(None, &path.into_inner(), body.into_inner());
        }

        api.group()
            .update(Some(&req_userid.0), &path.into_inner(), body.into_inner())
    })
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
/// delete endpoint for groups
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let id = path.into_inner();
    web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.group().delete(None, &id);
        }
        api.group().delete(Some(&req_userid.0), &id)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
