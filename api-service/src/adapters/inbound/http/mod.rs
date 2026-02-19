pub mod groups;

use std::sync::Arc;

use actix_web::{App, HttpResponse, Responder, get, web};
use tracing::debug;
use utoipa::ToSchema;

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
        web::scope("/api/v1").service(ping).service(
            web::scope("/groups")
                .service(groups::create)
                .service(groups::list)
                .service(groups::get)
                .service(groups::update)
                .service(groups::delete),
        ),
    )
}
#[utoipa::path(
    responses(
        (status = 200, description = "pong"),
    ),
)]
#[get("/ping")]
async fn ping() -> impl Responder {
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

#[derive(serde::Serialize, ToSchema)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

fn err_body(code: &'static str, message: String) -> ErrorBody {
    ErrorBody { code, message }
}
