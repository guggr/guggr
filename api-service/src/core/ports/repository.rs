use std::collections::HashMap;

use database_client::models::{
    Group, Job, JobDetailsHttp, JobDetailsPing, RefreshToken, User, UserGroupMapping,
};

use crate::core::{
    domain::errors::DomainError,
    models::{
        group::DisplayGroupMember,
        job::{
            JobWithRawDetails, UpdateJob, http::detail::UpdateJobDetailsHttp,
            ping::detail::UpdateJobDetailsPing, run::DisplayJobRun,
        },
    },
};

/// RepositoryPort trait which requires all repository port traits.
pub trait RepositoryPort:
    RepositoryUserPort
    + RepositoryRefreshTokenPort
    + RepositoryGroupPort
    + RepositoryUserGroupMappingPort
    + RepositoryJobRunPort
    + RepositoryJobPort
    + RepositoryJobDetailPort
{
}

pub trait RepositoryUserPort: Send + Sync {
    /// Inserts the user into the repository.
    fn create_user(&self, new_user: User) -> Result<User, DomainError>;

    /// Returns the user from the repository by ID.
    fn get_user(&self, id: &str) -> Result<User, DomainError>;

    /// Returns the user from the repository by ID.
    fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
}

pub trait RepositoryRefreshTokenPort: Send + Sync {
    /// Inserts the refresh token into the repository.
    fn create_refresh_token(&self, new_token: RefreshToken) -> Result<RefreshToken, DomainError>;

    /// Returns the refresh token from the repository by token.
    fn get_refresh_token(&self, token: &str) -> Result<RefreshToken, DomainError>;

    /// Deletes the refresh token from the repository by token.
    fn delete_refresh_token(&self, token: &str) -> Result<(), DomainError>;
}

pub trait RepositoryGroupPort: Send + Sync {
    /// Inserts the group into the repository.
    fn create_group(&self, new_group: Group) -> Result<Group, DomainError>;

    /// Returns the group from the repository by ID.
    fn get_group(&self, id: &str) -> Result<Group, DomainError>;

    /// Returns the groups from the repository by the user ID.
    fn list_groups_by_user_id(&self, user_id: &str) -> Result<Vec<Group>, DomainError>;

    /// Returns group members from the repository by group ids.
    fn get_members_for_multiple_groups(
        &self,
        group_ids: &[&str],
    ) -> Result<HashMap<String, Vec<DisplayGroupMember>>, DomainError>;
}

pub trait RepositoryUserGroupMappingPort: Send + Sync {
    /// Inserts the userGroupMapping into the repository.
    fn create_user_group_mapping(&self, new_mapping: UserGroupMapping) -> Result<(), DomainError>;
}

pub trait RepositoryJobRunPort: Send + Sync {
    /// Returns the JobRuns from the repository.
    fn list_job_run_results(
        &self,
        job_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DisplayJobRun>, DomainError>;
}

pub trait RepositoryJobPort: Send + Sync {
    /// Check if the user is part of the job's group
    fn check_user_job_group_membership(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<bool, DomainError>;

    fn check_user_job_edit_permissions(
        &self,
        user_id: &str,
        job_id: &str,
    ) -> Result<bool, DomainError>;

    fn check_user_can_create_job(&self, user_id: &str, group_id: &str)
    -> Result<bool, DomainError>;

    fn create_job(&self, new_job: Job) -> Result<Job, DomainError>;
    fn get_job_by_id(&self, user_id: &str, job_id: &str) -> Result<JobWithRawDetails, DomainError>;
    fn list_jobs(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<JobWithRawDetails>, DomainError>;
    fn delete_job(&self, job_id: &str) -> Result<(), DomainError>;
    fn update_job(&self, job_id: &str, updated_job: UpdateJob) -> Result<Job, DomainError>;
}

pub trait RepositoryJobDetailPort: Send + Sync {
    fn create_job_detail_http(
        &self,
        new_detail: JobDetailsHttp,
    ) -> Result<JobDetailsHttp, DomainError>;
    fn create_job_detail_ping(
        &self,
        new_detail: JobDetailsPing,
    ) -> Result<JobDetailsPing, DomainError>;

    fn get_job_detail_http(&self, detail_id: &str) -> Result<JobDetailsHttp, DomainError>;
    fn get_job_detail_ping(&self, detail_id: &str) -> Result<JobDetailsPing, DomainError>;

    fn update_job_detail_http(
        &self,
        detail_id: &str,
        update_detail: UpdateJobDetailsHttp,
    ) -> Result<JobDetailsHttp, DomainError>;
    fn update_job_detail_ping(
        &self,
        detail_id: &str,
        update_detail: UpdateJobDetailsPing,
    ) -> Result<JobDetailsPing, DomainError>;
}
