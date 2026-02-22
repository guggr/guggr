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
    core::{domain::errors::StorageError, models::auth::UserAuth, ports::storage::AuthOperations},
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
}
