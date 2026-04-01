use database_client::{
    models::{Job, JobResultHttp, JobResultPing, JobRun},
    schema::{job, job_result_http, job_result_ping, job_runs},
};
use diesel::{ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl};
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
        let selected_job: Job = job::dsl::job
            .find(job_id)
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        match selected_job.job_type_id.as_str() {
            "http" => {
                let http_rows: Vec<(JobRun, Option<JobResultHttp>)> = job_runs::table
                    .left_join(job_result_http::table)
                    .filter(job_runs::job_id.eq(job_id))
                    .select((
                        job_runs::all_columns,
                        job_result_http::all_columns.nullable(),
                    ))
                    .order(job_runs::timestamp.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(&mut conn)
                    .map_err(PostgresError::from)?;

                Ok(http_rows
                    .into_iter()
                    .map(|(job_row, http_row)| {
                        let mut job = DisplayJobRun::from(job_row);
                        job.details =
                            http_row.map(|row| DisplayJobRunDetails::Http(row.transmogrify()));
                        job
                    })
                    .collect())
            }
            "ping" => {
                let ping_rows: Vec<(JobRun, Option<JobResultPing>)> = job_runs::table
                    .left_join(job_result_ping::table)
                    .filter(job_runs::job_id.eq(job_id))
                    .select((
                        job_runs::all_columns,
                        job_result_ping::all_columns.nullable(),
                    ))
                    .order(job_runs::timestamp.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(&mut conn)
                    .map_err(PostgresError::from)?;

                Ok(ping_rows
                    .into_iter()
                    .map(|(job_row, ping_row)| {
                        let mut job = DisplayJobRun::from(job_row);
                        job.details =
                            ping_row.map(|row| DisplayJobRunDetails::Ping(row.transmogrify()));
                        job
                    })
                    .collect())
            }
            _ => Err(DomainError::NotFound),
        }
    }

    fn count_job_run_results(&self, job_id: &str) -> Result<i64, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let count: i64 = job_runs::table
            .filter(job_runs::job_id.eq(job_id))
            .count()
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(count)
    }
}
