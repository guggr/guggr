pub mod adapters;
pub mod core;
pub mod telemetry;

use crate::{
    adapters::outgoing::postgres::PostgresAdapter,
    core::{
        models::group::{CreateGroup, DisplayGroup, UpdateGroup},
        ports::storage::CrudOperations,
    },
};

/// TODO delete this
pub async fn example_usage(postgres: PostgresAdapter) -> anyhow::Result<()> {
    let group = CreateGroup {
        name: "This group is cool".to_string(),
    };
    postgres.group.create(group).await?;

    let r: Option<DisplayGroup> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let updated = UpdateGroup {
        name: Some("other description".to_string()),
    };

    postgres.group.update("mycoolgroup", updated).await?;

    let r: Option<DisplayGroup> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let entries: Vec<DisplayGroup> = postgres.group.list(5).await?;
    dbg!(entries);

    // this is kind of ugly, need to check how this can be done better
    postgres.group.delete("mycoolgroup").await?;

    let r: Option<DisplayGroup> = postgres.group.get_by_id("mycoolgroup").await?;
    dbg!(r);

    Ok(())
}
