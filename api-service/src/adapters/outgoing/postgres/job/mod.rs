use database_client::models::{self, Job};
use diesel::{
    PgConnection,
    dsl::exists,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use frunk::labelled::Transmogrifier;
pub mod detail;
pub mod run;
use crate::{
    adapters::outgoing::postgres::{
        PostgresAdapterError,
        job::{
            detail::{http::PostgresJobHttpAdapter, ping::PostgresJobPingAdapter},
            run::PostgresJobRunAdapter,
        },
    },
    core::{
        domain::errors::StorageError,
        models::job::{
            CreateJob, CreateJobDetails, DisplayJob, DisplayJobDetails, UpdateJob,
            UpdateJobDetails, UpdateableJob, http::detail::to_job_detail_http,
            ping::detail::to_job_detail_ping,
        },
        ports::storage::{JobCrudOperations, JobDetailOperations, JobRunCrudOperations},
    },
};

/// Sub-adapter of `PostgresAdapter`. Handles CRUD for the `role` table
pub struct PostgresJobAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
    pub run: PostgresJobRunAdapter,
    ping: PostgresJobPingAdapter,
    http: PostgresJobHttpAdapter,
}

impl PostgresJobAdapter {
    #[must_use]
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            pool: pool.clone(),
            run: PostgresJobRunAdapter::new(pool.clone()),
            ping: PostgresJobPingAdapter::new(pool.clone()),
            http: PostgresJobHttpAdapter::new(pool),
        }
    }
}

impl JobCrudOperations for PostgresJobAdapter {
    fn create(
        &self,
        user_id: Option<&str>,
        new_value: CreateJob,
    ) -> Result<DisplayJob, StorageError> {
        use database_client::schema::{job, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        if let Some(u_id) = user_id {
            let allowed: bool = diesel::select(exists(
                job::table
                    .inner_join(
                        user_group_mapping::table
                            .on(user_group_mapping::group_id.eq(job::group_id)),
                    )
                    .filter(job::id.eq(new_value.group_id.clone()))
                    .filter(user_group_mapping::user_id.eq(u_id)),
            ))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
            if !allowed {
                return Err(StorageError::Unauthorized);
            }
        };
        let new_job: models::Job = diesel::insert_into(job::dsl::job)
            .values(Job::from(new_value.clone()))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        let detail = match new_value.details {
            CreateJobDetails::Http(d) => {
                DisplayJobDetails::Http(self.http.create(to_job_detail_http(&new_job.id, d))?)
            }
            CreateJobDetails::Ping(d) => {
                DisplayJobDetails::Ping(self.ping.create(to_job_detail_ping(&new_job.id, d))?)
            }
        };
        let mut r: DisplayJob = DisplayJob::from(new_job);
        r.details = detail;
        Ok(r)
    }

    fn update(
        &self,
        user_id: Option<&str>,
        id: &str,
        update_value: UpdateJob,
    ) -> Result<DisplayJob, StorageError> {
        use database_client::schema::{job, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;
        if let Some(u_id) = user_id {
            let new_group = update_value.group_id.clone().unwrap_or(id.to_string());
            let allowed: bool = diesel::select(exists(
                job::table
                    .inner_join(
                        user_group_mapping::table
                            .on(user_group_mapping::group_id.eq(job::group_id)),
                    )
                    .filter(job::id.eq(id).and(job::id.eq(new_group)))
                    .filter(user_group_mapping::user_id.eq(u_id)),
            ))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
            if !allowed {
                return Err(StorageError::Unauthorized);
            }
        };
        let updated_job: models::Job = diesel::update(job::dsl::job.find(id))
            .set(UpdateableJob::from(update_value.clone()))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        if let Some(details) = update_value.details {
            let d = match details {
                UpdateJobDetails::Http(d) => DisplayJobDetails::Http(self.http.update(id, d)?),
                UpdateJobDetails::Ping(d) => DisplayJobDetails::Ping(self.ping.update(id, d)?),
            };
            let mut r: DisplayJob = DisplayJob::from(updated_job);
            r.details = d;
            return Ok(r);
        }

        // job details were not updated
        let details = if let Some(detail) = self.http.get_by_id(&updated_job.id)? {
            Some(DisplayJobDetails::Http(detail.transmogrify()))
        } else {
            self.ping
                .get_by_id(&updated_job.id)?
                .map(|detail| DisplayJobDetails::Ping(detail.transmogrify()))
        };

        let mut r: DisplayJob = DisplayJob::from(updated_job);
        r.details = details.unwrap_or(Err(PostgresAdapterError::NotFound)?);
        Ok(r)
    }

    fn get_by_id(
        &self,
        user_id: Option<&str>,
        id: &str,
    ) -> Result<Option<DisplayJob>, StorageError> {
        use database_client::schema::{
            job, job_details_http, job_details_ping, user_group_mapping,
        };

        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        if let Some(u_id) = user_id {
            let allowed: bool = diesel::select(exists(
                job::table
                    .inner_join(
                        user_group_mapping::table
                            .on(user_group_mapping::group_id.eq(job::group_id)),
                    )
                    .filter(job::id.eq(id))
                    .filter(user_group_mapping::user_id.eq(u_id)),
            ))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
            if !allowed {
                return Err(StorageError::Unauthorized);
            }
        };

        let http_row: Option<(models::Job, models::JobDetailsHttp)> = job::table
            .inner_join(job_details_http::table.on(job_details_http::id.eq(job::id)))
            .filter(job::id.eq(id))
            .select((job::all_columns, job_details_http::all_columns))
            .first(&mut conn)
            .optional()
            .map_err(PostgresAdapterError::from)?;

        if let Some((job_row, http_row)) = http_row {
            let mut job = DisplayJob::from(job_row);
            job.details = DisplayJobDetails::Http(http_row.transmogrify());
            return Ok(Some(job));
        }

        let ping_row: Option<(models::Job, models::JobDetailsPing)> = job::table
            .inner_join(job_details_ping::table.on(job_details_ping::id.eq(job::id)))
            .filter(job::id.eq(id))
            .select((job::all_columns, job_details_ping::all_columns))
            .first(&mut conn)
            .optional()
            .map_err(PostgresAdapterError::from)?;

        if let Some((job_row, ping_row)) = ping_row {
            let mut job = DisplayJob::from(job_row);
            job.details = DisplayJobDetails::Ping(ping_row.transmogrify());
            return Ok(Some(job));
        }

        Ok(None)
    }

    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), StorageError> {
        use database_client::schema::{job, user_group_mapping};
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        if let Some(u_id) = user_id {
            let allowed: bool = diesel::select(exists(
                job::table
                    .inner_join(
                        user_group_mapping::table
                            .on(user_group_mapping::group_id.eq(job::group_id)),
                    )
                    .filter(job::id.eq(id))
                    .filter(user_group_mapping::user_id.eq(u_id)),
            ))
            .get_result(&mut conn)
            .map_err(PostgresAdapterError::from)?;
            if !allowed {
                return Err(StorageError::Unauthorized);
            }
        };

        diesel::delete(job::dsl::job.filter(job::dsl::id.eq(id)))
            .execute(&mut conn)
            .map_err(PostgresAdapterError::from)?;
        // the detail tables have CASCADE ON DELETE set
        Ok(())
    }

    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<DisplayJob>, StorageError> {
        use database_client::schema::{
            job, job_details_http, job_details_ping, user_group_mapping,
        };
        let mut conn = self.pool.get().map_err(PostgresAdapterError::from)?;

        let http_rows: Vec<(models::Job, models::JobDetailsHttp)> = if let Some(u_id) = user_id {
            job::table
                .inner_join(
                    user_group_mapping::table.on(user_group_mapping::group_id.eq(job::group_id)),
                )
                .filter(user_group_mapping::user_id.eq(u_id))
                .inner_join(job_details_http::table.on(job_details_http::id.eq(job::id)))
                .select((job::all_columns, job_details_http::all_columns))
                .limit(limit)
                .load(&mut conn)
                .map_err(PostgresAdapterError::from)?
        } else {
            job::table
                .inner_join(job_details_http::table.on(job_details_http::id.eq(job::id)))
                .select((job::all_columns, job_details_http::all_columns))
                .limit(limit)
                .load(&mut conn)
                .map_err(PostgresAdapterError::from)?
        };
        if !http_rows.is_empty() {
            return Ok(http_rows
                .into_iter()
                .map(|(job_row, http_row)| {
                    let mut job = DisplayJob::from(job_row);
                    job.details = DisplayJobDetails::Http(http_row.transmogrify());

                    job
                })
                .collect());
        }

        let ping_rows: Vec<(models::Job, models::JobDetailsPing)> = if let Some(u_id) = user_id {
            job::table
                .inner_join(
                    user_group_mapping::table.on(user_group_mapping::group_id.eq(job::group_id)),
                )
                .filter(user_group_mapping::user_id.eq(u_id))
                .inner_join(job_details_ping::table.on(job_details_ping::id.eq(job::id)))
                .select((job::all_columns, job_details_ping::all_columns))
                .limit(limit)
                .load(&mut conn)
                .map_err(PostgresAdapterError::from)?
        } else {
            job::table
                .inner_join(job_details_ping::table.on(job_details_ping::id.eq(job::id)))
                .select((job::all_columns, job_details_ping::all_columns))
                .limit(limit)
                .load(&mut conn)
                .map_err(PostgresAdapterError::from)?
        };

        if !ping_rows.is_empty() {
            return Ok(ping_rows
                .into_iter()
                .map(|(job_row, http_row)| {
                    let mut job = DisplayJob::from(job_row);
                    job.details = DisplayJobDetails::Ping(http_row.transmogrify());

                    job
                })
                .collect());
        }

        Err(StorageError::NotFound)
    }

    fn run(&self) -> &(dyn JobRunCrudOperations + Send + Sync) {
        &self.run
    }
}
