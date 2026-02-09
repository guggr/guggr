//! Postgres adapter for fetching jobs from the database.

use async_trait::async_trait;
use database_client::{
    DbError,
    models::{Job, JobDetailsHttp, JobDetailsPing},
    schema::{
        job::dsl::{job, last_scheduled, run_every},
        job_details_http, job_details_ping,
    },
};
use diesel::{
    PgConnection,
    dsl::now,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use thiserror::Error;
use tracing::error;

use crate::core::{
    domain::{errors::JobRepositoryError, type_mapper::DatabaseJobResult},
    ports::job_fetcher::JobFetcher,
};

pub struct PostgresFetcher {
    pool: Pool<ConnectionManager<PgConnection>>,
}

/// Errors for [`PostgresFetcher`]
///
/// - [`PostgresFetcherError::ConnectionError`] is raised, when the initial
///   connection to the database failed, or the migrations could not be run. For
///   more information see [`DbError`]
/// - [`PostgresFetcherError::PoolGetConnectionError`] is raised, when no
///   connection could be obtained from the connection pool
/// - [`PostgresFetcherError::FetchJobsError`] is raised, when there was an
///   error fetching the jobs for scheduling
#[derive(Error, Debug)]
pub enum PostgresFetcherError {
    #[error("Database connection failed: {0}")]
    ConnectionError(#[from] DbError),
    #[error("Pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] diesel::r2d2::PoolError),
    #[error("Failed to fetch jobs for scheduling: {0}")]
    FetchJobsError(#[from] diesel::result::Error),
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresFetcherError> for JobRepositoryError {
    fn from(value: PostgresFetcherError) -> Self {
        match value {
            PostgresFetcherError::ConnectionError(_)
            | PostgresFetcherError::PoolGetConnectionError(_) => Self::Unavailable,

            other @ PostgresFetcherError::FetchJobsError { .. } => {
                Self::Internal(other.to_string())
            }
        }
    }
}

impl PostgresFetcher {
    /// Constructs a new [`PostgresFetcher`] instance from a given connection
    /// url.
    ///
    /// Invokes the [`database_client::create_connection_pool`] method.
    pub fn new(connection_url: &str) -> Result<Self, PostgresFetcherError> {
        let connection_pool = database_client::create_connection_pool(connection_url)?;

        Ok(Self {
            pool: connection_pool,
        })
    }

    /// Runs the query for fetching jobs from the database that are to be
    /// scheduled.
    ///
    /// Checks for unset [`last_scheduled`] field or where the [`run_every`]
    /// interval is exceeded.
    fn run_fetch_jobs_query(&self) -> Result<Vec<DatabaseJobResult>, PostgresFetcherError> {
        let mut conn = self.pool.get()?;

        // TODO: This is scaling extraordinary bad, maybe come up with sth better here?
        let db_jobs: Vec<DatabaseJobResult> = job
            .filter(
                last_scheduled
                    .is_null()
                    .or((last_scheduled.assume_not_null() + run_every).le(now)),
            )
            .left_join(job_details_http::table)
            .left_join(job_details_ping::table)
            .for_update()
            .skip_locked()
            .load(&mut conn)?;

        Ok(db_jobs)
    }
}

#[async_trait]
impl JobFetcher for PostgresFetcher {
    async fn fetch_jobs_batch(&self) -> Result<Vec<DatabaseJobResult>, JobRepositoryError> {
        Ok(self.run_fetch_jobs_query().map_err(|err| {
            error!("Database Error: {:?}", err);
            JobRepositoryError::from(err)
        })?)
    }
}
