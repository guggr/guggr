pub mod group;
use async_trait::async_trait;
use database_client::{DbError, create_connection_pool};
use thiserror::Error;

use crate::{
    adapters::outgoing::postgres::group::PostgresGroupAdapter,
    core::{domain::errors::StorageError, ports::storage::StoragePort},
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

#[async_trait]
impl StoragePort for PostgresAdapter {
    type GroupCrud = PostgresGroupAdapter;
    fn group(&self) -> &Self::GroupCrud {
        &self.group
    }
}
