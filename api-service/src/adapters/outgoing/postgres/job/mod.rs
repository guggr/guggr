pub mod run;

use database_client::schema::{job, user_group_mapping};
use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl, dsl::exists};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryJobPort},
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
}
