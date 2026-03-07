use database_client::{
    models::{JobDetailsHttp, JobDetailsPing},
    schema::{job_details_http, job_details_ping},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{
        domain::errors::DomainError,
        models::job::{http::detail::UpdateJobDetailsHttp, ping::detail::UpdateJobDetailsPing},
        ports::repository::RepositoryJobDetailPort,
    },
};

impl RepositoryJobDetailPort for Postgres {
    fn create_job_detail_http(
        &self,
        new_detail: JobDetailsHttp,
    ) -> Result<JobDetailsHttp, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let detail = diesel::insert_into(job_details_http::dsl::job_details_http)
            .values(new_detail)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }

    fn create_job_detail_ping(
        &self,
        new_detail: JobDetailsPing,
    ) -> Result<JobDetailsPing, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let detail = diesel::insert_into(job_details_ping::dsl::job_details_ping)
            .values(new_detail)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }

    fn get_job_detail_http(&self, detail_id: &str) -> Result<JobDetailsHttp, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let detail: JobDetailsHttp = job_details_http::table
            .filter(job_details_http::id.eq(detail_id))
            .first(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }

    fn get_job_detail_ping(&self, detail_id: &str) -> Result<JobDetailsPing, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let detail: JobDetailsPing = job_details_ping::table
            .filter(job_details_ping::id.eq(detail_id))
            .first(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }

    fn update_job_detail_http(
        &self,
        detail_id: &str,
        update_detail: UpdateJobDetailsHttp,
    ) -> Result<JobDetailsHttp, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let detail = diesel::update(job_details_http::dsl::job_details_http.find(detail_id))
            .set(update_detail)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }

    fn update_job_detail_ping(
        &self,
        detail_id: &str,
        update_detail: UpdateJobDetailsPing,
    ) -> Result<JobDetailsPing, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let detail = diesel::update(job_details_ping::dsl::job_details_ping.find(detail_id))
            .set(update_detail)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(detail)
    }
}
