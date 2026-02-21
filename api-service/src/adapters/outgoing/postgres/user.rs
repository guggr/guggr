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
        models::user::{CreateUser, DisplayUser, UpdateUser},
        ports::storage::CrudOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `user` table
pub struct PostgresUserAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresUserAdapter {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CrudOperations<CreateUser, UpdateUser, DisplayUser> for PostgresUserAdapter {
    async fn create(&self, new_value: CreateUser) -> Result<DisplayUser, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = models::User::try_from(new_value).map_err(StorageError::from)?;
        let result: models::User = diesel::insert_into(user)
            .values(u)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    async fn update(
        &self,
        id: &str,
        update_value: UpdateUser,
    ) -> Result<DisplayUser, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let result: models::User = diesel::update(user.find(id))
            .set(&update_value)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<DisplayUser>, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match user.find(id).first::<models::User>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    async fn delete(&self, id: &str) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::{self, group};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(group.filter(dsl::id.eq(id)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    async fn list(&self, limit: i64) -> Result<Vec<DisplayUser>, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let groups: Vec<models::User> = user
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(groups.into_iter().map(|u| u.transmogrify()).collect())
    }
}
