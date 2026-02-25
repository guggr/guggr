use database_client::models::{self, Role};
use diesel::{
    PgConnection,
    dsl::exists,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use frunk::labelled::Transmogrifier;

use crate::{
    adapters::outgoing::postgres::PostgresAdapterError,
    core::{
        domain::errors::StorageError,
        models::{
            auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth, UserAuthJwt},
            role::DisplayRole,
        },
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

impl AuthOperations for PostgresAuthAdapter {
    fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError> {
        use database_client::schema::user::dsl::{self, user};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = user
            .filter(dsl::email.eq(email))
            .first::<models::User>(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(u.transmogrify())
    }

    fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError> {
        use database_client::schema::refresh_token::dsl::refresh_token;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::insert_into(refresh_token)
            .values(models::RefreshToken::try_from(token)?)
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError> {
        use database_client::schema::refresh_token::dsl::{self, refresh_token};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let r = refresh_token
            .filter(dsl::jti.eq(jti))
            .first::<models::RefreshToken>(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(DisplayRefreshToken::from(r))
    }
    fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError> {
        use database_client::schema::refresh_token::dsl::{self, refresh_token};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(refresh_token.filter(dsl::jti.eq(jti)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }
    fn get_user_jwt_secrets(&self, id: &str) -> Result<UserAuthJwt, StorageError> {
        use database_client::schema::user::dsl::{self, user};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = user
            .filter(dsl::id.eq(id))
            .first::<models::User>(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(u.transmogrify())
    }
    fn get_roles_by_user(&self, id: &str) -> Result<Vec<DisplayRole>, StorageError> {
        use database_client::schema::{role, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let roles: Vec<Role> = user_group_mapping::table
            .inner_join(role::table)
            .filter(user_group_mapping::user_id.eq(id))
            .select((role::id, role::name))
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(roles.iter().map(|f| f.clone().transmogrify()).collect())
    }
    fn is_owner(&self, id: &str) -> Result<bool, StorageError> {
        use database_client::schema::{role, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let owner: bool = diesel::select(exists(
            user_group_mapping::table
                .inner_join(role::table)
                .filter(user_group_mapping::user_id.eq(id))
                .filter(role::id.eq("owner")),
        ))
        .get_result(&mut conn)
        .map_err(PostgresAdapterError::from)?;
        Ok(owner)
    }
}
