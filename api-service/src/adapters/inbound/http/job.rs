use std::sync::Arc;

use actix_web::{
    HttpResponse, Responder, delete, error::ErrorInternalServerError, get, patch, post, web,
};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        models::job::{CreateJob, DisplayJob, UpdateJob, run::DisplayJobRun},
        ports::storage::StoragePort,
    },
};

pub fn configure(cfg: &mut ServiceConfig) {
    let scope = utoipa_actix_web::scope("/jobs")
        .wrap(Auth)
        .service(create)
        .service(list)
        .service(get)
        .service(delete)
        .service(update)
        .service(list_runs);

    cfg.service(scope);
}

#[utoipa::path(
    request_body = CreateJob,
    operation_id = "create_job",
    responses(
        (status = 200, description = "Created group", body = DisplayJob),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[post("")]
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<CreateJob>,
) -> actix_web::Result<impl Responder> {
    let job = web::block(move || api.job().create(body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
        operation_id = "get_job",
    params(
        ("id" = String, Path, description = "Job id")
    ),
    responses(
        (status = 200, description = "Job", body = DisplayJob),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[get("/{id}")]
pub async fn get(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    web::block(move || api.job().get_by_id(&path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??
        .map_or_else(
            || Ok(HttpResponse::NotFound().finish()),
            |job| Ok(HttpResponse::Ok().json(job)),
        )
}

#[utoipa::path(
    request_body = UpdateJob,
    operation_id = "update_job",
    responses(
        (status = 200, description = "Created group", body = DisplayJob),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[patch("/{id}")]
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<UpdateJob>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let job = web::block(move || api.job().update(&path.into_inner(), body.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    operation_id = "update_job",
    params(
        ("id" = String, Path, description = "Job id")
    ),
    responses(
        (status = 204, description = "Deleted"),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[delete("/{id}")]
pub async fn delete(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    web::block(move || api.job().delete(&path.into_inner()))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(()))
}

#[utoipa::path(
    operation_id = "list_job",
    responses(
        (status = 200, description = "List jobs", body = [DisplayJob]),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[get("")]
pub async fn list(api: web::Data<Arc<dyn StoragePort>>) -> actix_web::Result<impl Responder> {
    let jobs = web::block(move || api.job().list(5))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(jobs))
}

#[utoipa::path(
    operation_id = "list_job_runs",
    params(
        ("id" = String, Path, description = "Job id")
    ),
    responses(
        (status = 200, description = "List jobs runs", body = [DisplayJobRun]),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[get("/{id}/runs")]
pub async fn list_runs(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let jobs = web::block(move || api.job().run().list_by_job_id(&path.into_inner(), 5))
        .await
        .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(jobs))
}
