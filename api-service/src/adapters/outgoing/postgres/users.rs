use database_client::{models::User, schema::user::dsl};
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl, FindDsl},
};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryUserPort},
};

impl RepositoryUserPort for Postgres {
    fn create_user(&self, new_user: User) -> Result<User, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let result: User = diesel::insert_into(dsl::user)
            .values(new_user)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(result)
    }

    fn get_user(&self, id: &str) -> Result<User, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let user = dsl::user
            .find(id)
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(user)
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let user = dsl::user
            .filter(dsl::email.eq(email))
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(user)
    }
}
