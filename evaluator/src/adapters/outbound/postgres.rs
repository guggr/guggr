use async_trait::async_trait;
use chrono::Utc;
use database_client::{
    DbError, create_connection_pool,
    models::{Job, JobRun},
    schema::job::id,
};
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use gen_proto_types::job_result::v1::JobResult;
use thiserror::Error;
use tracing::error;

use crate::core::{domain::errors::JobEvaluatorError, ports::database::DatabasePort};

pub struct PostgresAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

/// Errors for [`PostgresAdapter`]
///
/// - [`PostgresAdapterError::ConnectionError`] is raised, when the initial
///   connection to the database failed, or the migrations could not be run. For
///   more information see [`DbError`]
/// - [`PostgresAdapterError::PoolGetConnectionError`] is raised, when no
///   connection could be obtained from the connection pool
/// - [`PostgresAdapterError::ResultError`] is raised, when there was an error
///   while accessing the database
/// - [`PostgresAdapterError::UnknownJobId`] is raised, when the supplied Job ID
///   does not exist
#[derive(Error, Debug)]
pub enum PostgresAdapterError {
    #[error("Database connection failed: {0}")]
    ConnectionError(#[from] DbError),
    #[error("Pool exhausted or timeout: {0}")]
    PoolGetConnectionError(#[from] diesel::r2d2::PoolError),
    #[error("Failed to interact with the database: {0}")]
    ResultError(#[from] diesel::result::Error),
    #[error("Unknown Job ID: {0}")]
    UnknownJobId(String),
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresAdapterError> for JobEvaluatorError {
    fn from(value: PostgresAdapterError) -> Self {
        match value {
            PostgresAdapterError::ConnectionError(_)
            | PostgresAdapterError::PoolGetConnectionError(_) => Self::Unavailable,

            other => Self::Internal(other.to_string()),
        }
    }
}

impl PostgresAdapter {
    pub fn new(database_url: &str) -> Result<Self, PostgresAdapterError> {
        Ok(Self {
            pool: create_connection_pool(database_url)?,
        })
    }

    fn run_notification_enabled(&self, job_id: &str) -> Result<bool, PostgresAdapterError> {
        use database_client::schema::job::dsl::job;

        let mut conn = self.pool.get()?;

        let record: Option<Job> = job.filter(id.eq(job_id)).first(&mut conn).optional()?;
        match record {
            Some(job_record) => Ok(job_record.notify_users),
            None => Err(PostgresAdapterError::UnknownJobId(job_id.to_string())),
        }
    }

    fn run_write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> Result<(), PostgresAdapterError> {
        use database_client::schema::job_runs;

        let mut conn = self.pool.get()?;

        let job_run = JobRun {
            id: job_result.run_id.clone(),
            job_id: job_result.id.clone(),
            batch_id: job_result.batch_id.clone(),
            triggered_notification: notified,
            timestamp: Utc::now().naive_utc(), //TODO
            output: None,
        };

        diesel::insert_into(job_runs::table)
            .values(&job_run)
            .execute(&mut conn)?;
        Ok(())
    }
}

#[async_trait]
impl DatabasePort for PostgresAdapter {
    async fn notification_enabled(&self, job_id: &str) -> Result<bool, JobEvaluatorError> {
        self.run_notification_enabled(job_id).map_err(|err| {
            error!("Database Error: {:?}", err);
            JobEvaluatorError::from(err)
        })
    }

    async fn write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> Result<(), JobEvaluatorError> {
        self.run_write_job_result(job_result, notified)
            .map_err(|err| {
                error!("Database Error: {:?}", err);
                JobEvaluatorError::from(err)
            })
    }
}
