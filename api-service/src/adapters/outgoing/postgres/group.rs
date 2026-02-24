use database_client::models;
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
        models::group::{CreateGroup, DisplayGroup, UpdateGroup},
        ports::storage::RestrictedCrudOperations,
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `group` table
pub struct PostgresGroupAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresGroupAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl RestrictedCrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for PostgresGroupAdapter {
    fn create(&self, new_value: CreateGroup) -> Result<DisplayGroup, StorageError> {
        use database_client::schema::group::dsl::group;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let result: models::Group = diesel::insert_into(group)
            .values(models::Group::from(new_value))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    fn update(
        &self,
        user_id: Option<&str>,
        id: &str,
        update_value: UpdateGroup,
    ) -> Result<DisplayGroup, StorageError> {
        use database_client::schema::{group, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        if let Some(u_id) = user_id {
            match diesel::update(
                group::table.filter(
                    group::id.eq(id).and(exists(
                        user_group_mapping::table.filter(
                            user_group_mapping::group_id
                                .eq(id)
                                .and(user_group_mapping::user_id.eq(u_id)),
                        ),
                    )),
                ),
            )
            .set(update_value)
            .get_result::<models::Group>(&mut conn)
            {
                Ok(row) => return Ok(row.transmogrify()),
                Err(diesel::result::Error::NotFound) => {
                    return Err(PostgresAdapterError::NotFound)?;
                }
                Err(e) => return Err(PostgresAdapterError::from(e).into()),
            };
        }

        let result: models::Group = diesel::update(group::dsl::group.find(id))
            .set(&update_value)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    fn get_by_id(
        &self,
        user_id: Option<&str>,
        id: &str,
    ) -> Result<Option<DisplayGroup>, StorageError> {
        use database_client::schema::{group, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        if let Some(u_id) = user_id {
            match group::table
                .inner_join(
                    user_group_mapping::table.on(user_group_mapping::group_id.eq(group::id)),
                )
                .filter(group::id.eq(id))
                .filter(user_group_mapping::user_id.eq(u_id))
                .select((group::id, group::name))
                .first::<models::Group>(&mut conn)
            {
                Ok(row) => return Ok(Some(row.transmogrify())),
                Err(diesel::result::Error::NotFound) => return Ok(None),
                Err(e) => return Err(PostgresAdapterError::from(e).into()),
            };
        }

        match group::dsl::group.find(id).first::<models::Group>(&mut conn) {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }

    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), StorageError> {
        use database_client::schema::{group, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        if let Some(u_id) = user_id {
            diesel::delete(
                group::table.filter(
                    group::id.eq(id).and(exists(
                        user_group_mapping::table.filter(
                            user_group_mapping::group_id
                                .eq(id)
                                .and(user_group_mapping::user_id.eq(u_id)),
                        ),
                    )),
                ),
            )
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        } else {
            diesel::delete(group::dsl::group.filter(group::dsl::id.eq(id)))
                .execute(&mut conn)
                .map_err(PostgresAdapterError::from)?;
        }
        Ok(())
    }

    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<DisplayGroup>, StorageError> {
        use database_client::schema::{group, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let groups = if let Some(u_id) = user_id {
            group::table
                .inner_join(
                    user_group_mapping::table.on(user_group_mapping::group_id.eq(group::id)),
                )
                .filter(user_group_mapping::user_id.eq(u_id))
                .select((group::id, group::name))
                .distinct()
                .order(group::name.asc())
                .load::<models::Group>(&mut conn)
                .map_err(PostgresAdapterError::from)?
        } else {
            group::dsl::group
                .limit(limit)
                .load(&mut conn)
                .map_err(PostgresAdapterError::from)?
        };

        Ok(groups
            .into_iter()
            .map(frunk::labelled::Transmogrifier::transmogrify)
            .collect())
    }
}
