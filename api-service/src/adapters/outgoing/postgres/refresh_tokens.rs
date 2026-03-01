use database_client::{models::RefreshToken, schema::refresh_token::dsl};
use diesel::{
    ExpressionMethods, RunQueryDsl,
    query_dsl::methods::{FilterDsl, FindDsl},
};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{domain::errors::DomainError, ports::repository::RepositoryRefreshTokenPort},
};

impl RepositoryRefreshTokenPort for Postgres {
    fn create_refresh_token(&self, new_token: RefreshToken) -> Result<RefreshToken, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let result: RefreshToken = diesel::insert_into(dsl::refresh_token)
            .values(new_token)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(result)
    }

    fn get_refresh_token(&self, token: &str) -> Result<RefreshToken, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let user = dsl::refresh_token
            .find(token)
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(user)
    }

    fn delete_refresh_token(&self, token: &str) -> Result<(), DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        diesel::delete(dsl::refresh_token.filter(dsl::token.eq(token)))
            .execute(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(())
    }
}
