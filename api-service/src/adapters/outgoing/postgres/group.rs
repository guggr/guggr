use async_trait::async_trait;
use database_client::models::Group;
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

use crate::{
    adapters::outgoing::postgres::PostgresAdapterError,
    core::{domain::errors::StorageError, ports::storage::Crud},
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
impl Crud<Group> for PostgresGroupAdapter {
    async fn create(&self, new_value: Group) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::insert_into(group)
            .values(new_value)
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(())
    }

    async fn update(&self, update_value: Group) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::{group, name};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::update(group.find(update_value.id))
            .set(name.eq(update_value.name))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Group>, StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match group.find(id).first(&mut conn) {
            Ok(row) => Ok(Some(row)),
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

    async fn list(&self, limit: i64) -> Result<Vec<Group>, StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        Ok(group
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?)
    }
}
