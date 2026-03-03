use crate::core::{
    domain::errors::DomainError, models::job::run::DisplayJobRun,
    ports::service::ServiceJobRunPort, services::Service,
};

impl ServiceJobRunPort for Service {
    fn list_job_runs(
        &self,
        user_id: crate::core::models::auth::UserId,
        job_id: &str,
    ) -> Result<Vec<DisplayJobRun>, DomainError> {
        if !self
            .db
            .check_user_job_group_membership(&user_id.0, job_id)?
        {
            return Err(DomainError::NotFound);
        }
        // TODO handle pagination, see #96
        self.db.list_job_run_results(job_id, 10, 0)
    }
}
