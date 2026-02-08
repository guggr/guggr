//! Postgres adapter for fetching jobs from the database.

use async_trait::async_trait;
use chrono::Duration;
use database_client::{
    DbError,
    schema::job::dsl::{job, last_scheduled, run_every},
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
    domain::{self, errors::JobRepositoryError},
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
    #[error("Failed to fetch jobs for scheduling: {source}")]
    FetchJobsError { source: diesel::result::Error },
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
    fn run_fetch_jobs_query(
        &self,
    ) -> Result<Vec<database_client::models::Job>, PostgresFetcherError> {
        let mut conn = self.pool.get()?;

        let db_jobs: Vec<database_client::models::Job> = job
            .filter(
                last_scheduled
                    .is_null()
                    .or((last_scheduled.assume_not_null() + run_every).le(now)),
            )
            .for_update()
            .skip_locked()
            .load(&mut conn)
            .map_err(|e| PostgresFetcherError::FetchJobsError { source: e })?;

        Ok(db_jobs)
    }
}

impl TryFrom<database_client::models::Job> for domain::models::Job {
    type Error = JobRepositoryError;

    /// Converts database model [`database_client::models::Job`] to domain
    /// model. Mainly converts [`diesel::data_types::PgInterval`] to
    /// [`chrono::Duration`].
    fn try_from(value: database_client::models::Job) -> Result<Self, Self::Error> {
        const MICROSECONDS_PER_DAY: i64 = 24 * 60 * 60 * 1_000_000;

        // Months in Postgres are fuzzy, so this is more of a best guess
        let total_micros = value.run_every.microseconds
            + (i64::from(value.run_every.days) * MICROSECONDS_PER_DAY)
            + (i64::from(value.run_every.months) * MICROSECONDS_PER_DAY * 30);

        if total_micros < 0 {
            return Err(JobRepositoryError::Internal(
                "Negative interval found in database".into(),
            ));
        }

        Ok(Self {
            id: value.id,
            group_id: value.group_id,
            name: value.name,
            job_type_id: value.job_type_id,
            notify_users: value.notify_users,
            run_every: Duration::microseconds(total_micros),
            custom_notification: value.custom_notification,
            last_scheduled: value.last_scheduled,
        })
    }
}

#[async_trait]
impl JobFetcher for PostgresFetcher {
    async fn fetch_jobs_batch(&self) -> Result<Vec<domain::models::Job>, JobRepositoryError> {
        let db_jobs = self.run_fetch_jobs_query().map_err(|err| {
            error!("Database Error: {:?}", err);
            JobRepositoryError::from(err)
        })?;

        let domain_jobs: Vec<domain::models::Job> = db_jobs
            .into_iter()
            .map(domain::models::Job::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(domain_jobs)
    }
}
