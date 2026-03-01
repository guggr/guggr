//! Postgres adapter for deleting expired refresh tokens.

use async_trait::async_trait;
use database_client::{DbError, schema::refresh_token};
use diesel::{
    PgConnection,
    dsl::now,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;
use tracing::debug;

use crate::core::{domain::errors::JobSchedulerError, ports::token_cleaner::TokenCleaner};

pub struct PostgresTokenCleaner {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[derive(Error, Debug)]
pub enum PostgresTokenCleanerError {
    #[error("database connection failed: {0}")]
    ConnectionError(#[from] DbError),
    #[error("pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] diesel::r2d2::PoolError),
    #[error("failed to delete expired tokens: {0}")]
    DeleteError(#[from] diesel::result::Error),
}

impl From<PostgresTokenCleanerError> for JobSchedulerError {
    fn from(value: PostgresTokenCleanerError) -> Self {
        match value {
            PostgresTokenCleanerError::ConnectionError(e) => {
                Self::DatabaseUnavailable(e.to_string())
            }
            PostgresTokenCleanerError::PoolGetConnectionError(e) => {
                Self::DatabaseUnavailable(e.to_string())
            }
            other @ PostgresTokenCleanerError::DeleteError { .. } => {
                Self::Internal(other.to_string())
            }
        }
    }
}

impl PostgresTokenCleaner {
    pub fn new(connection_url: &str) -> Result<Self, PostgresTokenCleanerError> {
        let pool = database_client::create_connection_pool(connection_url)?;
        Ok(Self { pool })
    }

    fn run_delete_query(&self) -> Result<usize, PostgresTokenCleanerError> {
        debug!("getting db connection");
        let mut conn = self.pool.get()?;

        debug!("deleting expired refresh tokens");
        let count = diesel::delete(refresh_token::table)
            .filter(refresh_token::expires_on.lt(now))
            .execute(&mut conn)?;

        Ok(count)
    }
}

#[async_trait]
impl TokenCleaner for PostgresTokenCleaner {
    async fn delete_expired_tokens(&self) -> Result<usize, JobSchedulerError> {
        self.run_delete_query().map_err(|err| {
            tracing::error!("database error: {:?}", err);
            JobSchedulerError::from(err)
        })
    }
}
