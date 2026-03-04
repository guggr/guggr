use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, delete, error::ErrorInternalServerError,
    get, patch, post, web,
};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::{errors::DomainError, openapi_helper},
        models::{
            auth::UserId,
            job::{CreateJob, DisplayJob, UpdateRequestJob, run::DisplayJobRun},
        },
        ports::service::ServicePort,
    },
};

/// configures all paths under the subpath `/jobs`
pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/jobs")
        .wrap(Auth)
        .service(list_runs)
        .service(list)
        .service(create)
        .service(update)
        .service(get)
        .service(delete);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateJob,
    operation_id = "create_job",
    responses(
        (status = 200, description = "Job successfully created", body = DisplayJob),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[post("")]
/// Create Job
pub async fn create(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: web::Json<CreateJob>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let job = web::block(move || svc.create_job(auth_user, body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    request_body = UpdateRequestJob,
    operation_id = "update_job",
    responses(
        (status = 200, description = "Job successfully updated", body = DisplayJob),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[patch("/{id}")]
/// Update Job
pub async fn update(
    svc: web::Data<Arc<dyn ServicePort>>,
    body: web::Json<UpdateRequestJob>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let job = web::block(move || svc.update_job(auth_user, &path.into_inner(), body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    operation_id = "get_job",
    responses(
        (status = 200, description = "Job", body = DisplayJob),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResNotFound,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[get("/{id}")]
/// Get Job
pub async fn get(
    svc: web::Data<Arc<dyn ServicePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let job = web::block(move || svc.get_job_by_id(auth_user, &path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    operation_id = "list_job",
    responses(
        (status = 200, description = "Jobs", body = [DisplayJob]),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResInternalServerError,
    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[get("")]
/// List Jobs
pub async fn list(
    svc: web::Data<Arc<dyn ServicePort>>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    let job = web::block(move || svc.list_jobs(auth_user))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
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
/// List Job Runs
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

#[utoipa::path(
    operation_id = "delete_job",
    params(
        ("id" = String, Path, description = "Job ID")
    ),
    responses(
        (status = 204, description = "Deleted"),
        openapi_helper::ResUnauthorized,
        openapi_helper::ResInternalServerError,


    ),
    security(("token" = [])),
    tag = "jobs"
)]
#[delete("/{id}")]
/// Delete Job
pub async fn delete(
    svc: web::Data<Arc<dyn ServicePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let auth_user = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(DomainError::Unauthorized)?;
    web::block(move || svc.delete_job(auth_user, &path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(()))
}
