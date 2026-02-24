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
        models::user::{CreateUser, DisplayUser, UpdateUser, UpdateableUser},
        ports::storage::CrudOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `user` table
pub struct PostgresUserAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresUserAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl CrudOperations<CreateUser, UpdateUser, DisplayUser> for PostgresUserAdapter {
    fn create(&self, new_value: CreateUser) -> Result<DisplayUser, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = models::User::try_from(new_value).map_err(StorageError::from)?;
        let result: models::User = diesel::insert_into(user)
            .values(u)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    fn update(&self, id: &str, update_value: UpdateUser) -> Result<DisplayUser, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = UpdateableUser::try_from(update_value).map_err(StorageError::from)?;
        let result: models::User = diesel::update(user.find(id))
            .set(&u)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    fn get_by_id(&self, id: &str) -> Result<Option<DisplayUser>, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match user.find(id).first::<models::User>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    fn delete(&self, id: &str) -> Result<(), StorageError> {
        use database_client::schema::group::dsl::{self, group};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(group.filter(dsl::id.eq(id)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    fn list(&self, limit: i64) -> Result<Vec<DisplayUser>, StorageError> {
        use database_client::schema::user::dsl::user;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let users: Vec<models::User> = user
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(users
            .into_iter()
            .map(frunk::labelled::Transmogrifier::transmogrify)
            .collect())
    }
}
