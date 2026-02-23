pub mod auth;
pub mod group;
pub mod role;
pub mod user;
use async_trait::async_trait;
use database_client::{DbError, create_connection_pool};
use thiserror::Error;

use crate::{
    adapters::outgoing::postgres::{
        auth::PostgresAuthAdapter, group::PostgresGroupAdapter, role::PostgresRoleAdapter,
        user::PostgresUserAdapter,
    },
    core::{
        domain::errors::StorageError,
        models::{
            group::{CreateGroup, DisplayGroup, UpdateGroup},
            role::{CreateRole, DisplayRole, UpdateRole},
            user::{CreateUser, DisplayUser, UpdateUser},
        },
        ports::storage::{AuthOperations, CrudOperations, StoragePort},
    },
};
pub struct PostgresAdapter {
    pub group: PostgresGroupAdapter,
    pub user: PostgresUserAdapter,
    pub role: PostgresRoleAdapter,
    pub auth: PostgresAuthAdapter,
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
    Result(diesel::result::Error),
    /// Raised, when no record was found
    #[error("Record not Found")]
    NotFound,
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
            PostgresAdapterError::NotFound => Self::NotFound,
            other => Self::Internal(other.to_string()),
        }
    }
}

/// Allows for converting the argon2-specific errors to domain errors
impl From<argon2::password_hash::Error> for StorageError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

/// Allows for converting the diesel-specific errors to domain errors
/// used instead of #[from] for more control
impl From<diesel::result::Error> for PostgresAdapterError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Self::NotFound,
            other => Self::Result(other),
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
            group: PostgresGroupAdapter::new(pool.clone()),
            user: PostgresUserAdapter::new(pool.clone()),
            role: PostgresRoleAdapter::new(pool.clone()),
            auth: PostgresAuthAdapter::new(pool),
        })
    }
}

#[async_trait]
impl StoragePort for PostgresAdapter {
    fn group(&self) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync) {
        &self.group
    }
    fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync) {
        &self.user
    }
    fn role(&self) -> &(dyn CrudOperations<CreateRole, UpdateRole, DisplayRole> + Send + Sync) {
        &self.role
    }
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync) {
        &self.auth
    }
}
