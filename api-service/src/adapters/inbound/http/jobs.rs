use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, error::ErrorInternalServerError, get, web,
};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::{errors::DomainError, openapi_helper},
        models::{auth::UserId, job::run::DisplayJobRun},
        ports::service::ServicePort,
    },
};

/// configures all paths under the subpath `/jobs`
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/jobs")
        .wrap(Auth)
        .service(list_runs);

    cfg.service(scope);
}

#[utoipa::path(
    operation_id = "list_job_runs",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "List jobs runs", body = [DisplayJobRun]),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[get("/{id}/runs")]
/// list endpoint for job runs
pub async fn list_runs(
    svc: web::Data<Arc<dyn ServicePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let runs = web::block(move || svc.list_job_runs(auth_user, &path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(runs))
}
