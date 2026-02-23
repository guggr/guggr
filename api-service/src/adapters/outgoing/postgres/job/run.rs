use database_client::models;
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use frunk::labelled::Transmogrifier;

use crate::{
    adapters::outgoing::postgres::PostgresAdapterError,
    core::{
        domain::errors::StorageError,
        models::job::run::{DisplayJobRun, DisplayJobRunDetails},
        ports::storage::JobRunCrudOperations,
    },
};

/// Sub-adapter of `PostgresJobAdapter`. Handles CRUD for the `job_runs`,
/// `job_result_ping` and `job_result_http` tables
pub struct PostgresJobRunAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresJobRunAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl JobRunCrudOperations for PostgresJobRunAdapter {
    fn get_by_job_id(&self, job_id: &str) -> Result<Option<DisplayJobRun>, StorageError> {
        use database_client::schema::{job_result_http, job_result_ping, job_runs};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let http_row: Option<(models::JobRun, models::JobResultHttp)> = job_runs::table
            .inner_join(job_result_http::table.on(job_result_http::id.eq(job_runs::id)))
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_http::all_columns))
            .first(&mut conn)
            .optional()
            .map_err(PostgresAdapterError::from)?;

        if let Some((job_row, http_row)) = http_row {
            let mut job = DisplayJobRun::from(job_row);
            job.details = DisplayJobRunDetails::Http(http_row.transmogrify());
            return Ok(Some(job));
        }

        let ping_row: Option<(models::JobRun, models::JobResultPing)> = job_runs::table
            .inner_join(job_result_ping::table.on(job_result_ping::id.eq(job_runs::id)))
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_ping::all_columns))
            .first(&mut conn)
            .optional()
            .map_err(PostgresAdapterError::from)?;

        if let Some((job_row, ping_row)) = ping_row {
            let mut job = DisplayJobRun::from(job_row);
            job.details = DisplayJobRunDetails::Ping(ping_row.transmogrify());
            return Ok(Some(job));
        }
        Ok(None)
    }
    fn list_by_job_id(&self, job_id: &str, limit: i64) -> Result<Vec<DisplayJobRun>, StorageError> {
        use database_client::schema::{job_result_http, job_result_ping, job_runs};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let http_rows: Vec<(models::JobRun, models::JobResultHttp)> = job_runs::table
            .inner_join(job_result_http::table.on(job_result_http::id.eq(job_runs::id)))
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_http::all_columns))
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        if !http_rows.is_empty() {
            return Ok(http_rows
                .into_iter()
                .map(|(job_row, http_row)| {
                    let mut job = DisplayJobRun::from(job_row);
                    job.details = DisplayJobRunDetails::Http(http_row.transmogrify());
                    job
                })
                .collect());
        }

        let ping_rows: Vec<(models::JobRun, models::JobResultPing)> = job_runs::table
            .inner_join(job_result_ping::table.on(job_result_ping::id.eq(job_runs::id)))
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_ping::all_columns))
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        if !ping_rows.is_empty() {
            return Ok(ping_rows
                .into_iter()
                .map(|(job_row, row)| {
                    let mut job = DisplayJobRun::from(job_row);
                    job.details = DisplayJobRunDetails::Ping(row.transmogrify());
                    job
                })
                .collect());
        }

        Err(StorageError::NotFound)
    }
}
