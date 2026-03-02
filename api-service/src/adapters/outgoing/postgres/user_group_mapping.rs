use database_client::{models::UserGroupMapping, schema::user_group_mapping};
use diesel::RunQueryDsl;

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryUserGroupMappingPort},
};

impl RepositoryUserGroupMappingPort for Postgres {
    fn create_user_group_mapping(&self, new_mapping: UserGroupMapping) -> Result<(), DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        diesel::insert_into(user_group_mapping::dsl::user_group_mapping)
            .values(new_mapping)
            .execute(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(())
    }
}
