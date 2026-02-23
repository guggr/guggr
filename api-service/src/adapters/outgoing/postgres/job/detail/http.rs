use database_client::models::{self, JobDetailsHttp};
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
        models::job::http::detail::{DisplayJobDetailsHttp, UpdateJobDetailsHttp},
        ports::storage::JobDetailOperations,
    },
};

/// Sub-adapter of `PostgresJobAdapter`. Handles CRU for the `job_details_http`
/// table
pub struct PostgresJobHttpAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresJobHttpAdapter {
    #[must_use]
    pub const fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }
}

impl JobDetailOperations<UpdateJobDetailsHttp, DisplayJobDetailsHttp> for PostgresJobHttpAdapter {
    fn create(
        &self,
        new_value: DisplayJobDetailsHttp,
    ) -> Result<DisplayJobDetailsHttp, StorageError> {
        use database_client::schema::job_details_http::dsl::job_details_http;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        let n: JobDetailsHttp = new_value.transmogrify();
        let result: models::JobDetailsHttp = diesel::insert_into(job_details_http)
            .values(n)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;

        Ok(result.transmogrify())
    }

    fn update(
        &self,
        id: &str,
        update_value: UpdateJobDetailsHttp,
    ) -> Result<DisplayJobDetailsHttp, StorageError> {
        use database_client::schema::job_details_http::dsl::job_details_http;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let result: models::JobDetailsHttp = diesel::update(job_details_http.find(id))
            .set(&update_value)
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        Ok(result.transmogrify())
    }
    fn get_by_id(&self, id: &str) -> Result<Option<DisplayJobDetailsHttp>, StorageError> {
        use database_client::schema::job_details_http::dsl::job_details_http;
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        match job_details_http
            .find(id)
            .first::<models::JobDetailsHttp>(&mut conn)
        {
            Ok(row) => Ok(Some(row.transmogrify())),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(e) => Err(PostgresAdapterError::from(e).into()),
        }
    }
}
