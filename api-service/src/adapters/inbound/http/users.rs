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
        domain::{
            errors::{AuthError, StorageError},
            openapi_helper::{GenericResponses, GenericResponsesCU},
        },
        models::{
            auth::UserId,
            user::{CreateUser, DisplayUser, UpdateUser},
        },
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
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let user = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.user().create(body.into_inner());
        }
        Err(StorageError::Unauthorized)
    })
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
pub async fn list(
    api: web::Data<Arc<dyn StoragePort>>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let users = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            api.user().list(5)
        } else {
            let u = api.user().get_by_id(&req_userid.0)?;
            if let Some(user) = u {
                return Ok(vec![user]);
            }
            Ok(vec![])
        }
    })
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
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    web::block(move || {
        let id = path.into_inner();
        if api.auth().is_owner(&req_userid.0)? || req_userid.0 == id {
            return api.user().get_by_id(&id);
        }
        Err(StorageError::Unauthorized)
    })
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
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let user = web::block(move || {
        let id = path.into_inner();
        if api.auth().is_owner(&req_userid.0)? || req_userid.0 == id {
            return api.user().update(&id, body.into_inner());
        }
        Err(StorageError::Unauthorized)
    })
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
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    web::block(move || {
        let id = path.into_inner();
        if api.auth().is_owner(&req_userid.0)? || req_userid.0 == id {
            return api.user().delete(&id);
        }
        Err(StorageError::Unauthorized)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::NoContent().finish())
}
