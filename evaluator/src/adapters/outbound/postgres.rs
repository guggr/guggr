use async_trait::async_trait;
use chrono::Utc;
use database_client::{
    create_connection_pool,
    models::{Job, JobRun},
    schema::job::id,
};
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use gen_proto_types::job_result::v1::JobResult;
use nanoid::nanoid;

use crate::core::{ports::database::DatabasePort, service::evalservice::EvalServiceError};

pub struct PostgresAdapter {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresAdapter {
    pub fn new(database_url: &str) -> Result<Self, anyhow::Error> {
        Ok(Self {
            pool: create_connection_pool(database_url)?,
        })
    }
}

#[async_trait]
impl DatabasePort for PostgresAdapter {
    async fn notification_enabled(&self, job_id: &str) -> anyhow::Result<bool, EvalServiceError> {
        use database_client::schema::job::dsl::job;

        let mut conn = self.pool.get()?;

        let record: Option<Job> = job
            .filter(id.eq(job_id))
            .first(&mut conn)
            .optional()
            .map_err(EvalServiceError::from)?;
        match record {
            Some(job_record) => Ok(job_record.notify_users),
            None => Err(EvalServiceError::UnknownJobId),
        }
    }

    async fn write_job_result(
        &self,
        job_result: &JobResult,
        notified: bool,
    ) -> anyhow::Result<(), EvalServiceError> {
        use database_client::schema::job_runs;

        let mut conn = self.pool.get()?;

        let job_run = JobRun {
            id: nanoid!(),
            job_id: job_result.id.clone(),
            triggered_notification: notified,
            timestamp: Utc::now().naive_utc(),
            output: None,
        };

        diesel::insert_into(job_runs::table)
            .values(&job_run)
            .execute(&mut conn)?;
        Ok(())
    }
}
