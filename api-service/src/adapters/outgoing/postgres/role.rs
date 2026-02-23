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
        models::role::{CreateRole, DisplayRole, UpdateRole},
        ports::storage::CrudOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `role` table
pub struct PostgresRoleAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresRoleAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl CrudOperations<CreateRole, UpdateRole, DisplayRole> for PostgresRoleAdapter {
    fn create(&self, new_value: CreateRole) -> Result<DisplayRole, StorageError> {
        use database_client::schema::role::dsl::role;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let u = models::Role::from(new_value);
        let result: models::Role = diesel::insert_into(role)
            .values(u)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    fn update(&self, id: &str, update_value: UpdateRole) -> Result<DisplayRole, StorageError> {
        use database_client::schema::role::dsl::role;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let result: models::Role = diesel::update(role.find(id))
            .set(&update_value)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    fn get_by_id(&self, id: &str) -> Result<Option<DisplayRole>, StorageError> {
        use database_client::schema::role::dsl::role;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match role.find(id).first::<models::Role>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    fn delete(&self, id: &str) -> Result<(), StorageError> {
        use database_client::schema::role::dsl::{self, role};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        diesel::delete(role.filter(dsl::id.eq(id)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(())
    }

    fn list(&self, limit: i64) -> Result<Vec<DisplayRole>, StorageError> {
        use database_client::schema::role::dsl::role;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let roles: Vec<models::Role> = role
            .limit(limit)
            .load(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(roles
            .into_iter()
            .map(frunk::labelled::Transmogrifier::transmogrify)
            .collect())
    }
}
