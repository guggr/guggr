use async_trait::async_trait;
use database_client::{DbError, create_connection_pool, models::Group};
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;

use crate::core::{
    domain::errors::StorageError,
    ports::storage::{Crud, StoragePort},
};

pub struct PostgresAdapter {
    pub group: PostgresGroupAdapter,
}

/// Errors for [`PostgresAdapter`]
#[derive(Error, Debug)]
pub enum PostgresAdapterError {
    /// Raised, when the initial connection to the database failed, or the
    /// migrations could not be run. For more information see [`DbError`]
    #[error("Database connection failed: {0}")]
    Connection(#[from] DbError),
    /// Raised, when no connection could be obtained from the connection pool
    #[error("Pool exhausted or timeout: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),
    /// Raised, when there was an error while accessing the database
    #[error("Failed to interact with the database: {0}")]
    Result(#[from] diesel::result::Error),
    /// Raised, when the supplied Job ID does not exist
    #[error("Unknown Job ID: {0}")]
    UnknownJobId(String),
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresAdapterError> for StorageError {
    fn from(value: PostgresAdapterError) -> Self {
        match value {
            PostgresAdapterError::Connection(e) => Self::Unavailable(e.to_string()),
            PostgresAdapterError::Pool(e) => Self::Unavailable(e.to_string()),

            other => Self::Unavailable(other.to_string()),
        }
    }
}

impl PostgresAdapter {
    /// Creates a new `PostgresAdapter`
    ///
    /// # Errors
    ///
    /// Will return [`PostgresAdapterError`] if no connection pool could be
    /// created from the supplied database url
    pub fn new(database_url: &str) -> Result<Self, PostgresAdapterError> {
        let pool = create_connection_pool(database_url)?;
        Ok(Self {
            group: PostgresGroupAdapter::new(pool),
        })
    }
}

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

#[async_trait]
impl StoragePort for PostgresAdapter {
    type GroupCrud = PostgresGroupAdapter;
    fn group(&self) -> &Self::GroupCrud {
        &self.group
    }
}
