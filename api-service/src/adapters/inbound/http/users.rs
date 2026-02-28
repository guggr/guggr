use std::sync::Arc;

use actix_web::{HttpRequest, HttpResponse, Responder, error::ErrorInternalServerError, post, web};
use garde_actix_web::web::Json;
use utoipa_actix_web::service_config::ServiceConfig;

use crate::core::{
    domain::openapi_helper,
    models::user::{CreateUser, DisplayUser},
    ports::service::ServicePort,
};

/// Configures all users endpoints
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/users").service(create);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateUser,
    operation_id = "create_user",
    responses(
        (status = 200, description = "User successfully created", body = DisplayUser),
        openapi_helper::ResBadRequest,
        openapi_helper::ResInternalServerError,
    ),
    tag = "users"
)]
#[post("")]
/// Create user
pub async fn create(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: Json<CreateUser>,
    _req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let user = web::block(move || svc.create_user(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;

    Ok(HttpResponse::Ok().json(user))
}
