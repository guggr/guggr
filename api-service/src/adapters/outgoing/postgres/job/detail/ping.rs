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
        models::job::ping::detail::{DisplayJobDetailsPing, UpdateJobDetailsPing},
        ports::storage::JobDetailOperations,
    },
};

/// Sub-adapter of `PostgresJobAdapter`. Handles CRU for the `job_details_ping`
/// table
pub struct PostgresJobPingAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresJobPingAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl JobDetailOperations<UpdateJobDetailsPing, DisplayJobDetailsPing> for PostgresJobPingAdapter {
    fn create(
        &self,
        new_value: DisplayJobDetailsPing,
    ) -> Result<DisplayJobDetailsPing, StorageError> {
        use database_client::schema::job_details_ping::dsl::job_details_ping;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let n: models::JobDetailsPing = new_value.transmogrify();

        let result: models::JobDetailsPing = diesel::insert_into(job_details_ping)
            .values(n)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    fn update(
        &self,
        id: &str,
        update_value: UpdateJobDetailsPing,
    ) -> Result<DisplayJobDetailsPing, StorageError> {
        use database_client::schema::job_details_ping::dsl::job_details_ping;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let result: models::JobDetailsPing = diesel::update(job_details_ping.find(id))
            .set(&update_value)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }

    fn get_by_id(&self, id: &str) -> Result<Option<DisplayJobDetailsPing>, StorageError> {
        use database_client::schema::job_details_ping::dsl::job_details_ping;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match job_details_ping
            .find(id)
            .first::<models::JobDetailsPing>(&mut conn)
        {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }
}
