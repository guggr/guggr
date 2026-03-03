use database_client::{
    models::{JobResultHttp, JobResultPing, JobRun},
    schema::{job_result_http, job_result_ping, job_runs},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use frunk::labelled::Transmogrifier;

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{
        domain::errors::DomainError,
        models::job::run::{DisplayJobRun, DisplayJobRunDetails},
        ports::repository::RepositoryJobRunPort,
    },
};

impl RepositoryJobRunPort for Postgres {
    fn list_job_run_results(
        &self,
        job_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DisplayJobRun>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let http_rows: Vec<(JobRun, JobResultHttp)> = job_runs::table
            .inner_join(job_result_http::table)
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_http::all_columns))
            .order(job_runs::timestamp.desc())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .map_err(PostgresError::from)?;

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

        let ping_rows: Vec<(JobRun, JobResultPing)> = job_runs::table
            .inner_join(job_result_ping::table)
            .filter(job_runs::job_id.eq(job_id))
            .select((job_runs::all_columns, job_result_ping::all_columns))
            .order(job_runs::timestamp.desc())
            .limit(limit)
            .offset(offset)
            .load(&mut conn)
            .map_err(PostgresError::from)?;

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
        Err(DomainError::NotFound)
    }
}
