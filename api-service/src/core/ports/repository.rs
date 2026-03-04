use std::collections::HashMap;

use database_client::models::{Group, RefreshToken, User, UserGroupMapping};

use crate::core::{
    domain::errors::DomainError,
    models::{group::DisplayGroupMember, job::run::DisplayJobRun},
};

/// RepositoryPort trait which requires all repository port traits.
pub trait RepositoryPort:
    RepositoryUserPort
    + RepositoryRefreshTokenPort
    + RepositoryGroupPort
    + RepositoryUserGroupMappingPort
    + RepositoryJobRunPort
    + RepositoryJobPort
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

    /// Returns the groups from the repository by the user ID.
    fn list_group_ids_by_user_id(&self, user_id: &str) -> Result<Vec<Group>, DomainError>;

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
}
