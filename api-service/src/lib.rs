pub mod adapters;
pub mod core;
pub mod telemetry;

use database_client::models::Group;

use crate::{adapters::outgoing::postgres::PostgresAdapter, core::ports::storage::Crud};

/// TODO delete this
pub async fn example_usage(postgres: PostgresAdapter) -> anyhow::Result<()> {
    let group = Group {
        id: "mycoolgroup".to_string(),
        name: "This group is cool".to_string(),
    };
    postgres.create(group).await?;

    let r: Option<Group> = postgres.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let updategroup = Group {
        id: "mycoolgroup".to_string(),
        name: "other description".to_string(),
    };

    postgres.update(updategroup).await?;

    let r: Option<Group> = postgres.get_by_id("mycoolgroup").await?;
    dbg!(r);

    let entries: Vec<Group> = postgres.list(5).await?;
    dbg!(entries);

    // this is kind of ugly, need to check how this can be done better
    <PostgresAdapter as Crud<Group>>::delete(&postgres, "mycoolgroup").await?;

    let r: Option<Group> = postgres.get_by_id("mycoolgroup").await?;
    dbg!(r);

    Ok(())
}
