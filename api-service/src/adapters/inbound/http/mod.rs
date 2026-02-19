pub mod groups;

use std::sync::Arc;

use actix_web::{App, HttpResponse, Responder, web};
use tracing::debug;

use crate::core::{domain::errors::StorageError, ports::storage::StoragePort};

pub fn app(
    api: web::Data<Arc<dyn StoragePort>>,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    debug!("creating new app");
    App::new().app_data(api).service(
        web::scope("/api/v1")
            .route("/ping", web::get().to(health))
            .service(
                web::scope("/groups")
                    .route("", web::post().to(groups::create))
                    .route("", web::get().to(groups::list))
                    .route("/{id}", web::get().to(groups::get))
                    .route("/{id}", web::put().to(groups::update))
                    .route("/{id}", web::delete().to(groups::delete)),
            ),
    )
}

async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

fn map_storage_error(err: StorageError) -> HttpResponse {
    match err {
        StorageError::Internal(_) => HttpResponse::InternalServerError()
            .json(err_body("unexpected", "Something went wrong".to_string())),
        StorageError::Unavailable(_) => HttpResponse::InternalServerError()
            .json(err_body("unexpected", "Something went wrong".to_string())),
    }
}

#[derive(serde::Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

fn err_body(code: &'static str, message: String) -> ErrorBody {
    ErrorBody { code, message }
}
