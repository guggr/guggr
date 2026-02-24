use std::fs;

use actix_web::App;
use api_service::{
    adapters::inbound::http::{self, auth, groups, jobs, roles, users},
    core::domain::openapi_helper::ApiDoc,
};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_actix_web::{self, AppExt};

fn main() -> anyhow::Result<()> {
    let (_, a) = App::new()
        .wrap(TracingLogger::default())
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .service(
            utoipa_actix_web::scope("/api/v1")
                .configure(groups::configure)
                .configure(users::configure)
                .configure(jobs::configure)
                .configure(roles::configure)
                .configure(auth::configure)
                .configure(http::configure),
        )
        .split_for_parts();

    fs::write("api-service/openapi.json", a.to_pretty_json()?)?;
    Ok(())
}
