use database_client::models::Job;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::UserId,
        job::{
            CreateJob, CreateJobDetails, DisplayJob, DisplayJobDetails, UpdateRequestJob,
            UpdateRequestJobDetails, http::detail::CreateJobDetailsHttp,
            ping::detail::CreateJobDetailsPing,
        },
    },
    ports::service::ServiceJobPort,
    services::Service,
};
pub mod runs;

impl ServiceJobPort for Service {
    fn create_job(&self, user_id: UserId, new_job: CreateJob) -> Result<DisplayJob, DomainError> {
        if !self
            .db
            .check_user_can_create_job(&user_id.0, &new_job.group_id)?
        {
            return Err(DomainError::Unauthorized);
        }

        let job = self.db.create_job(Job::from(new_job.clone()))?;

        let detail = match new_job.details {
            CreateJobDetails::Http(d) => DisplayJobDetails::Http(
                self.db
                    .create_job_detail_http(CreateJobDetailsHttp::from_create_detail(&job.id, d))?
                    .transmogrify(),
            ),
            CreateJobDetails::Ping(d) => DisplayJobDetails::Ping(
                self.db
                    .create_job_detail_ping(CreateJobDetailsPing::from_create_detail(&job.id, d))?
                    .transmogrify(),
            ),
        };

        let mut display_job = DisplayJob::from(job);
        display_job.details = detail;
        Ok(display_job)
    }

    fn get_job_by_id(&self, user_id: UserId, job_id: &str) -> Result<DisplayJob, DomainError> {
        let raw_job = self.db.get_job_by_id(&user_id.0, job_id)?;
        let (job, http, ping) = raw_job;
        let details = match (http, ping) {
            (Some(http), None) => DisplayJobDetails::Http(http.transmogrify()),
            (None, Some(ping)) => DisplayJobDetails::Ping(ping.transmogrify()),
            (_, _) => DisplayJobDetails::Undefined,
        };
        let mut job = DisplayJob::from(job);
        job.details = details;
        Ok(job)
    }

    fn list_jobs(&self, user_id: UserId) -> Result<Vec<DisplayJob>, DomainError> {
        let raw_jobs = self.db.list_jobs(&user_id.0, 10, 0)?;
        Ok(raw_jobs
            .into_iter()
            .map(|(job, http, ping)| {
                let mut display_job = DisplayJob::from(job);
                if let Some(detail) = http {
                    display_job.details = DisplayJobDetails::Http(detail.transmogrify());
                } else if let Some(detail) = ping {
                    display_job.details = DisplayJobDetails::Ping(detail.transmogrify());
                };
                display_job
            })
            .collect())
    }

    fn update_job(
        &self,
        user_id: UserId,
        job_id: &str,
        updated_job: UpdateRequestJob,
    ) -> Result<DisplayJob, DomainError> {
        if !self
            .db
            .check_user_job_group_membership(&user_id.0, job_id)?
        {
            return Err(DomainError::NotFound);
        }

        if !self
            .db
            .check_user_job_edit_permissions(&user_id.0, job_id)?
        {
            return Err(DomainError::Unauthorized);
        }

        let job = self
            .db
            .update_job(job_id, updated_job.clone().transmogrify())?;

        let detail = match updated_job.details {
            Some(detail) => match detail {
                UpdateRequestJobDetails::Http(d) => DisplayJobDetails::Http(
                    self.db.update_job_detail_http(&job.id, d)?.transmogrify(),
                ),
                UpdateRequestJobDetails::Ping(d) => DisplayJobDetails::Ping(
                    self.db.update_job_detail_ping(&job.id, d)?.transmogrify(),
                ),
            },
            None => match job.job_type_id.as_str() {
                "http" => {
                    DisplayJobDetails::Http(self.db.get_job_detail_http(&job.id)?.transmogrify())
                }
                "ping" => {
                    DisplayJobDetails::Ping(self.db.get_job_detail_ping(&job.id)?.transmogrify())
                }
                _ => return Err(DomainError::NotFound),
            },
        };
        let mut job = DisplayJob::from(job);
        job.details = detail;

        Ok(job)
    }

    fn delete_job(&self, user_id: UserId, job_id: &str) -> Result<(), DomainError> {
        if !self
            .db
            .check_user_job_edit_permissions(&user_id.0, job_id)?
        {
            return Err(DomainError::Unauthorized);
        }
        self.db.delete_job(job_id)
    }
}
