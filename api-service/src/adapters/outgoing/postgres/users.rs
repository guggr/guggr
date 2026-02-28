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
        domain::errors::DomainError,
        models::user::{CreateUser, DisplayUser, UpdateUser},
        ports::storage::RestrictedCrudOperations,
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

impl RestrictedCrudOperations<CreateUser, UpdateUser, DisplayUser> for PostgresUserAdapter {
    fn create(&self, new_value: CreateUser) -> Result<DisplayUser, DomainError> {
        use database_client::schema::user::dsl::user;

        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let result: models::User = diesel::insert_into(user)
            .values(models::User::from(new_value))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    fn update(
        &self,
        _user_id: Option<&str>,
        _id: &str,
        _update_value: UpdateUser,
    ) -> Result<DisplayUser, DomainError> {
        // TODO implement
        Err(DomainError::Internal("not implemented".to_owned()))
    }

    fn get_by_id(
        &self,
        _user_id: Option<&str>,
        id: &str,
    ) -> Result<Option<DisplayUser>, DomainError> {
        use database_client::schema::user;

        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        match user::dsl::user.find(id).first::<models::User>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),

            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    fn delete(&self, _user_id: Option<&str>, _id: &str) -> Result<(), DomainError> {
        // TODO implement
        Err(DomainError::Internal("not implemented".to_owned()))
    }

    fn list(&self, _user_id: Option<&str>, _limit: i64) -> Result<Vec<DisplayUser>, DomainError> {
        Ok(vec![])
    }
}
