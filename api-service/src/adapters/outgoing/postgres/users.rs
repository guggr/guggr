use database_client::{models::User, schema::user::dsl::user};
use diesel::RunQueryDsl;

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryUserPort},
};

impl RepositoryUserPort for Postgres {
    fn create_user(&self, new_user: User) -> Result<User, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let result: User = diesel::insert_into(user)
            .values(new_user)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(result)
    }
}
