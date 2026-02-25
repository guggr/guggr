use std::sync::Arc;

use actix_web::{
    HttpMessage, HttpRequest, HttpResponse, Responder, delete, error::ErrorInternalServerError,
    get, patch, post, web,
};
use utoipa_actix_web::service_config::ServiceConfig;

use crate::{
    adapters::inbound::http::middleware::auth::Auth,
    core::{
        domain::errors::AuthError,
        models::{
            auth::UserId,
            job::{CreateJob, DisplayJob, UpdateJob, run::DisplayJobRun},
        },
        ports::storage::StoragePort,
    },
};

/// configures all paths under the subpath `/jobs`
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
        (status = 200, description = "Created job", body = DisplayJob),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[post("")]
/// create endpoint for jobs
pub async fn create(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<CreateJob>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let job = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.job().create(None, body.into_inner());
        }
        api.job().create(Some(&req_userid.0), body.into_inner())
    })
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
/// get endpoint for jobs
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
            return api.job().get_by_id(None, &path.into_inner());
        }
        api.job().get_by_id(Some(&req_userid.0), &path.into_inner())
    })
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
        (status = 200, description = "Created job", body = DisplayJob),
        (status = 500, description = "Storage error")
    ),
    security(("bearerAuth" = [])),
    tag = "jobs"
)]
#[patch("/{id}")]
/// update endpoint for jobs
pub async fn update(
    api: web::Data<Arc<dyn StoragePort>>,
    body: web::Json<UpdateJob>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let job = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api
                .job()
                .update(None, &path.into_inner(), body.into_inner());
        }
        api.job()
            .update(Some(&req_userid.0), &path.into_inner(), body.into_inner())
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(job))
}

#[utoipa::path(
    operation_id = "delete_job",
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
/// delete endpoint for jobs
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
        if api.auth().is_owner(&req_userid.0)? {
            return api.job().delete(None, &path.into_inner());
        }
        api.job().delete(Some(&req_userid.0), &path.into_inner())
    })
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
/// list endpoint for jobs
pub async fn list(
    api: web::Data<Arc<dyn StoragePort>>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let jobs = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.job().list(None, 5);
        }
        api.job().list(Some(&req_userid.0), 5)
    })
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
/// list endpoint for job runs
pub async fn list_runs(
    api: web::Data<Arc<dyn StoragePort>>,
    path: web::Path<String>,
    req: HttpRequest,
) -> actix_web::Result<impl Responder> {
    let req_userid = req
        .extensions()
        .get::<UserId>()
        .cloned()
        .ok_or(AuthError::Unauthorized)?;
    let jobs = web::block(move || {
        if api.auth().is_owner(&req_userid.0)? {
            return api.job().run().list_by_job_id(None, &path.into_inner(), 5);
        }
        api.job()
            .run()
            .list_by_job_id(Some(&req_userid.0), &path.into_inner(), 5)
    })
    .await
    .map_err(ErrorInternalServerError)??;
    Ok(HttpResponse::Ok().json(jobs))
}
