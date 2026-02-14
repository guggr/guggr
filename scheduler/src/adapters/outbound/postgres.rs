//! Postgres adapter for fetching jobs from the database.

use async_trait::async_trait;
use database_client::{
    DbError,
    schema::{
        job::{self, id, last_scheduled},
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
use tracing::{debug, error};

use crate::core::{
    domain::{errors::JobSchedulerError, type_mapper::DatabaseJob},
    ports::job_fetcher::JobFetcher,
};

pub struct PostgresFetcher {
    pool: Pool<ConnectionManager<PgConnection>>,
}

/// Errors for [`PostgresFetcher`]
#[derive(Error, Debug)]
pub enum PostgresFetcherError {
    /// Raised, when the initial connection to the database failed, or the
    /// migrations could not be run. For more information see [`DbError`].
    #[error("database connection failed: {0}")]
    ConnectionError(#[from] DbError),
    /// Raised, when no connection could be obtained from the connection pool.
    #[error("pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] diesel::r2d2::PoolError),
    /// Raised, when there was an error fetching the jobs for scheduling.
    #[error("failed to fetch jobs for scheduling: {0}")]
    FetchJobsError(#[from] diesel::result::Error),
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresFetcherError> for JobSchedulerError {
    fn from(value: PostgresFetcherError) -> Self {
        match value {
            PostgresFetcherError::ConnectionError(e) => Self::DatabaseUnavailable(e.to_string()),
            PostgresFetcherError::PoolGetConnectionError(e) => {
                Self::DatabaseUnavailable(e.to_string())
            }

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
    ///
    /// # Errors
    /// Returns an error if the connection pool creation failed.
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
    fn run_fetch_jobs_query(&self) -> Result<Vec<DatabaseJob>, PostgresFetcherError> {
        debug!("getting db connection");
        let mut conn = self.pool.get()?;

        debug!("loading ids to lock from db");

        // TODO: This is scaling extraordinary bad, maybe come up with sth better here?
        let ids_to_lock: Vec<String> = job::table
            .select(job::id)
            .filter(
                job::last_scheduled
                    .is_null()
                    .or((job::last_scheduled.assume_not_null() + job::run_every).le(now)),
            )
            .for_update()
            .skip_locked()
            .load(&mut conn)?;

        debug!("loading job details from db");
        let db_jobs = job::table
            .filter(id.eq_any(&ids_to_lock))
            .left_join(job_details_http::table)
            .left_join(job_details_ping::table)
            .load(&mut conn)?;

        debug!("setting last_scheduled timestamp in db to now");
        diesel::update(job::table)
            .filter(id.eq_any(ids_to_lock))
            .set(last_scheduled.eq(now))
            .execute(&mut conn)?;

        Ok(db_jobs)
    }
}

#[async_trait]
impl JobFetcher for PostgresFetcher {
    async fn fetch_jobs_batch(&self) -> Result<Vec<DatabaseJob>, JobSchedulerError> {
        Ok(self.run_fetch_jobs_query().map_err(|err| {
            error!("database error: {:?}", err);
            JobSchedulerError::from(err)
        })?)
    }
}
