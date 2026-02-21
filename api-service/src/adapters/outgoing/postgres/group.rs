use async_trait::async_trait;
use database_client::models;
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use frunk::labelled::Transmogrifier;

use crate::{
    adapters::outgoing::postgres::PostgresAdapterError,
    core::{
        domain::errors::StorageError,
        models::group::{CreateGroup, DisplayGroup, UpdateGroup},
        ports::storage::CrudOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `group` table
pub struct PostgresGroupAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresGroupAdapter {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for PostgresGroupAdapter {
    async fn create(&self, new_value: CreateGroup) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::insert_into(group)
            .values(models::Group::from(new_value))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(())
    }

    async fn update(&self, id: &str, update_value: UpdateGroup) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        diesel::update(group.find(id))
            .set(&update_value)
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<DisplayGroup>, StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match group.find(id).first::<models::Group>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    async fn delete(&self, id: &str) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::{self, group};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(group.filter(dsl::id.eq(id)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    async fn list(&self, limit: i64) -> Result<Vec<DisplayGroup>, StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let groups: Vec<models::Group> = group
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(groups.into_iter().map(|u| u.transmogrify()).collect())
    }
}
