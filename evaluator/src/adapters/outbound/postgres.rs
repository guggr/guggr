use async_trait::async_trait;
use database_client::{
    DbError, create_connection_pool,
    models::{Job, JobResultHttp, JobResultPing, JobRun},
    schema::job::id,
};
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use gen_proto_types::job_result::{
    types::v1::{HttpJobResult, PingJobResult},
    v1::JobResult,
};
use thiserror::Error;
use tracing::error;

use crate::{
    core::{domain::errors::JobEvaluatorError, ports::database::DatabasePort},
    ipnet_from_bytes_host, naive_from_proto_ts, protocheck_duration_to_i32_millis,
};

pub struct PostgresAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

/// Errors for [`PostgresAdapter`]
///
/// - [`PostgresAdapterError::Connection`] is raised, when the initial
///   connection to the database failed, or the migrations could not be run. For
///   more information see [`DbError`]
/// - [`PostgresAdapterError::Pool`] is raised, when no connection could be
///   obtained from the connection pool
/// - [`PostgresAdapterError::Result`] is raised, when there was an error while
///   accessing the database
/// - [`PostgresAdapterError::UnknownJobId`] is raised, when the supplied Job ID
///   does not exist
#[derive(Error, Debug)]
pub enum PostgresAdapterError {
    #[error("Database connection failed: {0}")]
    Connection(#[from] DbError),
    #[error("Pool exhausted or timeout: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),
    #[error("Failed to interact with the database: {0}")]
    Result(#[from] diesel::result::Error),
    #[error("Unknown Job ID: {0}")]
    UnknownJobId(String),
    #[error("No Result was attached to the JobResult: {0}")]
    NoResult(String),
    #[error("Could not convert IP/Timestamp: {0}")]
    Conversion(String),
}

/// Allows for converting the Postgres-specific errors to domain errors
impl From<PostgresAdapterError> for JobEvaluatorError {
    fn from(value: PostgresAdapterError) -> Self {
        match value {
            PostgresAdapterError::Connection(_) | PostgresAdapterError::Pool(_) => {
                Self::Unavailable
            }

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

    fn write_job_result_http(
        &self,
        run_id: &str,
        result: &HttpJobResult,
    ) -> Result<(), PostgresAdapterError> {
        use database_client::schema::job_result_http;
        let mut conn = self.pool.get()?;

        let result = JobResultHttp {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(&result.ip_address)
                .map_err(|err| PostgresAdapterError::Conversion(err.to_string()))?,
            status_code: result.status_code,
            latency: protocheck_duration_to_i32_millis(result.latency.unwrap())
                .map_err(|err| PostgresAdapterError::Conversion(err.to_string()))?,
            payload: result.payload.clone(),
        };
        diesel::insert_into(job_result_http::table)
            .values(&result)
            .execute(&mut conn)?;

        Ok(())
    }

    fn write_job_result_ping(
        &self,
        run_id: &str,
        result: &PingJobResult,
    ) -> Result<(), PostgresAdapterError> {
        use database_client::schema::job_result_ping;
        let mut conn = self.pool.get()?;

        let result = JobResultPing {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(result.ip_address.as_slice())
                .map_err(|err| PostgresAdapterError::Conversion(err.to_string()))?,
            latency: protocheck_duration_to_i32_millis(result.latency.unwrap())
                .map_err(|err| PostgresAdapterError::Conversion(err.to_string()))?,
        };
        diesel::insert_into(job_result_ping::table)
            .values(&result)
            .execute(&mut conn)?;

        Ok(())
    }

    fn run_write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> Result<(), PostgresAdapterError> {
        use database_client::schema::job_runs;

        let mut conn = self.pool.get()?;

        let reachable = if job_result.http.is_some() {
            job_result.http.as_ref().unwrap().reachable
        } else if job_result.ping.is_some() {
            job_result.ping.as_ref().unwrap().reachable
        } else {
            return Err(PostgresAdapterError::NoResult(job_result.run_id.clone()));
        };

        let job_run = JobRun {
            id: job_result.run_id.clone(),
            job_id: job_result.id.clone(),
            batch_id: job_result.batch_id.clone(),
            triggered_notification: notified,
            timestamp: naive_from_proto_ts(&job_result.timestamp.unwrap()).unwrap(),
            reachable,
        };

        diesel::insert_into(job_runs::table)
            .values(&job_run)
            .execute(&mut conn)?;

        if job_result.http.is_some() {
            self.write_job_result_http(&job_result.run_id, job_result.http.as_ref().unwrap())?;
        } else if job_result.ping.is_some() {
            self.write_job_result_ping(&job_result.run_id, job_result.ping.as_ref().unwrap())?;
        }
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
