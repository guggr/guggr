pub mod users;

use database_client::{DbError, create_connection_pool};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;

use crate::core::{domain::errors::DomainError, ports::repository::RepositoryPort};

pub struct Postgres {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Postgres {
    /// Creates a new [`Postgres`] adapter
    ///
    /// # Errors
    ///
    /// Returns [`PostgresError`] if no connection pool could be
    /// created from the supplied database URL
    pub fn new(database_url: &str) -> Result<Self, PostgresError> {
        let pool = create_connection_pool(database_url)?;

        Ok(Self { pool })
    }
}

impl RepositoryPort for Postgres {}

/// Errors for [`Postgres`]
#[derive(Error, Debug)]
pub enum PostgresError {
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
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresError> for DomainError {
    fn from(value: PostgresError) -> Self {
        match value {
            PostgresError::Connection(e) => Self::Unavailable(e.to_string()),
            PostgresError::Pool(e) => Self::Unavailable(e.to_string()),
            PostgresError::NotFound => Self::NotFound,
            other => Self::Internal(other.to_string()),
        }
    }
}

/// Allows for converting the argon2-specific errors to domain errors
impl From<argon2::password_hash::Error> for DomainError {
    fn from(value: argon2::password_hash::Error) -> Self {
        Self::Internal(value.to_string())
    }
}

/// Allows for converting the diesel-specific errors to domain errors
/// used instead of #[from] for more control
impl From<diesel::result::Error> for PostgresError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => Self::NotFound,
            other => Self::Result(other),
        }
    }
}
