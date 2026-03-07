pub mod detail;
pub mod run;

use database_client::{
    models::{Job, JobDetailsHttp, JobDetailsPing},
    schema::{job, job_details_http, job_details_ping, user_group_mapping},
};
use diesel::{
    ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl, RunQueryDsl, dsl::exists,
};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{
        domain::errors::DomainError,
        models::job::{JobWithRawDetails, UpdateJob},
        ports::repository::RepositoryJobPort,
    },
};

impl RepositoryJobPort for Postgres {
    fn check_user_job_group_membership(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<bool, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        Ok(diesel::select(exists(
            job::table
                .inner_join(
                    user_group_mapping::table.on(job::group_id.eq(user_group_mapping::group_id)),
                )
                .filter(job::id.eq(job_id))
                .filter(user_group_mapping::user_id.eq(user_id)),
        ))
        .get_result(&mut conn)
        .map_err(PostgresError::from)?)
    }

    fn check_user_job_edit_permissions(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<bool, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        Ok(diesel::select(exists(
            job::table
                .inner_join(
                    user_group_mapping::table.on(job::group_id.eq(user_group_mapping::group_id)),
                )
                .filter(user_group_mapping::role_id.eq_any(vec!["owner", "admin"]))
                .filter(job::id.eq(job_id))
                .filter(user_group_mapping::user_id.eq(user_id)),
        ))
        .get_result(&mut conn)
        .map_err(PostgresError::from)?)
    }

    fn check_user_can_create_job(
        &self,
        user_id: &str,
        group_id: &str,
    ) -> Result<bool, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        Ok(diesel::select(exists(
            user_group_mapping::table
                .filter(user_group_mapping::role_id.eq_any(vec!["owner", "admin"]))
                .filter(user_group_mapping::group_id.eq(group_id))
                .filter(user_group_mapping::user_id.eq(user_id)),
        ))
        .get_result(&mut conn)
        .map_err(PostgresError::from)?)
    }

    fn create_job(&self, new_job: Job) -> Result<Job, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        Ok(diesel::insert_into(job::dsl::job)
            .values(new_job)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?)
    }

    fn get_job_by_id(&self, user_id: &str, job_id: &str) -> Result<JobWithRawDetails, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let job: (Job, Option<JobDetailsHttp>, Option<JobDetailsPing>) = job::table
            .find(job_id)
            .inner_join(
                user_group_mapping::table.on(job::group_id.eq(user_group_mapping::group_id)),
            )
            .filter(user_group_mapping::user_id.eq(user_id))
            .left_join(job_details_http::table)
            .left_join(job_details_ping::table)
            .select((
                job::all_columns,
                job_details_http::all_columns.nullable(),
                job_details_ping::all_columns.nullable(),
            ))
            .first::<(Job, Option<JobDetailsHttp>, Option<JobDetailsPing>)>(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(job)
    }

    fn list_jobs(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<JobWithRawDetails>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let jobs: Vec<(Job, Option<JobDetailsHttp>, Option<JobDetailsPing>)> = job::table
            .inner_join(
                user_group_mapping::table.on(job::group_id.eq(user_group_mapping::group_id)),
            )
            .filter(user_group_mapping::user_id.eq(user_id))
            .left_join(job_details_http::table)
            .left_join(job_details_ping::table)
            .select((
                job::all_columns,
                job_details_http::all_columns.nullable(),
                job_details_ping::all_columns.nullable(),
            ))
            .limit(limit)
            .offset(offset)
            .load::<(Job, Option<JobDetailsHttp>, Option<JobDetailsPing>)>(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(jobs)
    }

    fn update_job(&self, job_id: &str, updated_job: UpdateJob) -> Result<Job, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let updated_job: Job = diesel::update(job::dsl::job.find(job_id))
            .set(updated_job)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(updated_job)
    }

    fn delete_job(&self, job_id: &str) -> Result<(), DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        diesel::delete(job::dsl::job.filter(job::dsl::id.eq(job_id)))
            .execute(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(())
    }
}
