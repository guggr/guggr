use async_trait::async_trait;
use database_client::models;
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use frunk::labelled::Transmogrifier;

use crate::{
    adapters::outgoing::postgres::PostgresAdapterError,
    core::{
        domain::errors::StorageError,
        models::auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth},
        ports::storage::AuthOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles auth requests
pub struct PostgresAuthAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresAuthAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthOperations for PostgresAuthAdapter {
    async fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError> {
        use database_client::schema::user::dsl::{self, user};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = user
            .filter(dsl::email.eq(email))
            .first::<models::User>(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(u.transmogrify())
    }

    async fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError> {
        use database_client::schema::refresh_token::dsl::refresh_token;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::insert_into(refresh_token)
            .values(models::RefreshToken::try_from(token)?)
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    async fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError> {
        use database_client::schema::refresh_token::dsl::{self, refresh_token};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let r = refresh_token
            .filter(dsl::jti.eq(jti))
            .first::<models::RefreshToken>(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(DisplayRefreshToken::from(r))
    }
    async fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError> {
        use database_client::schema::refresh_token::dsl::{self, refresh_token};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(refresh_token.filter(dsl::jti.eq(jti)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }
}
