pub mod adapters;
pub mod core;
pub mod telemetry;

use actix_web::App;
use database_client::models::Group;
use utoipa::openapi::OpenApi;
use utoipa_actix_web::{AppExt, scope};

use crate::{
    adapters::{
        inbound::http::{groups, ping},
        outgoing::postgres::PostgresAdapter,
    },
    core::ports::storage::CrudOperations,
};

/// TODO delete this
pub async fn example_usage(postgres: PostgresAdapter) -> anyhow::Result<()> {
    let group = Group {
        id: "mycoolgroup".to_string(),
        name: "This group is cool".to_string(),
    };
    postgres.group.create(group).await?;

    let r: Option<Group> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let updategroup = Group {
        id: "mycoolgroup".to_string(),
        name: "other description".to_string(),
    };

    postgres.group.update(updategroup).await?;

    let r: Option<Group> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let entries: Vec<Group> = postgres.group.list(5).await?;
    dbg!(entries);

    // this is kind of ugly, need to check how this can be done better
    postgres.group.delete("mycoolgroup").await?;

    let r: Option<Group> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    Ok(())
}

pub fn generate_open_api() -> OpenApi {
    let (_, api) = App::new()
        .into_utoipa_app()
        .service(
            scope::scope("/api/v1").service(ping).service(
                scope::scope("/groups")
                    .service(groups::create)
                    .service(groups::list)
                    .service(groups::get)
                    .service(groups::update)
                    .service(groups::delete),
            ),
        )
        .split_for_parts();
    api
}
