use database_client::{
    models::Group,
    schema::{group, user_group_mapping},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryGroupPort},
};

impl RepositoryGroupPort for Postgres {
    fn create_group(
        &self,
        new_group: database_client::models::Group,
    ) -> Result<database_client::models::Group, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let result: Group = diesel::insert_into(group::dsl::group)
            .values(new_group)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(result)
    }

    fn list_groups_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<database_client::models::Group>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let groups: Vec<Group> = group::table
            .inner_join(user_group_mapping::table)
            .filter(user_group_mapping::user_id.eq(user_id))
            .select((group::id, group::name))
            .distinct()
            .order(group::name.asc())
            .load::<Group>(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(groups)
    }
}
