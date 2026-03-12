use crate::core::{
    domain::errors::DomainError,
    models::{
        job::run::DisplayJobRun,
        pagination::{PaginatedResponse, PaginatedResponseMetadata, PaginationQuery},
    },
    ports::service::ServiceJobRunPort,
    services::Service,
};

impl ServiceJobRunPort for Service {
    fn list_job_runs(
        &self,
        pagination: &PaginationQuery,
        user_id: crate::core::models::auth::UserId,
        job_id: &str,
    ) -> Result<PaginatedResponse<DisplayJobRun>, DomainError> {
        if !self
            .db
            .check_user_job_group_membership(&user_id.0, job_id)?
        {
            return Err(DomainError::NotFound);
        }
        let runs = self.db.list_job_run_results(
            job_id,
            pagination.per_page.into(),
            pagination.page.into(),
        )?;
        let count = self.db.count_job_run_results(job_id)?;
        Ok(PaginatedResponse::new(
            runs,
            PaginatedResponseMetadata::build(pagination, count),
        ))
    }
}
