use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::{
            AuthenticatedResponse, LoginRequest, LogoutRequest, TokenRefreshRequest, TokenResponse,
            UserId,
        },
        group::{CreateGroup, DisplayGroup},
        job::{CreateJob, DisplayJob, UpdateRequestJob, run::DisplayJobRun},
        pagination::{PaginatedResponse, PaginationQuery},
        user::{CreateUser, DisplayUser},
    },
};

/// ServicePort trait which requires all service port traits.
pub trait ServicePort:
    ServiceUserPort + ServiceAuthPort + ServiceGroupPort + ServiceJobRunPort + ServiceJobPort
{
}

/// Service port including all user interactions.
pub trait ServiceUserPort: Send + Sync {
    /// Creates a new user
    fn create_user(&self, new_user: CreateUser) -> Result<DisplayUser, DomainError>;
    fn get_user(&self, auth_user: UserId, id: &str) -> Result<DisplayUser, DomainError>;
}

/// Service port for auth interactions.
pub trait ServiceAuthPort: Send + Sync {
    /// Validates the access token and returns the user ID if successful.
    fn validate_access_token(&self, token: &str) -> Result<String, DomainError>;
    fn login(&self, login_req: LoginRequest) -> Result<AuthenticatedResponse, DomainError>;

    /// Renews refresh and access tokens.
    fn refresh_auth_tokens(
        &self,
        refresh_req: TokenRefreshRequest,
    ) -> Result<TokenResponse, DomainError>;

    /// Invalidates the given refresh token.
    fn logout(&self, logout_req: LogoutRequest) -> Result<(), DomainError>;
}

/// Service port for group interactions.
pub trait ServiceGroupPort: Send + Sync {
    /// Create a new group and set the supplied user as group owner
    fn create_group(
        &self,
        user_id: UserId,
        new_group: CreateGroup,
    ) -> Result<DisplayGroup, DomainError>;
    // Gets group by group ID
    fn get_group(&self, user_id: UserId, id: &str) -> Result<DisplayGroup, DomainError>;
    /// List groups by the supplied user ID
    fn list_groups_by_user(&self, user_id: UserId) -> Result<Vec<DisplayGroup>, DomainError>;
}

/// Service port for job run interactions.
pub trait ServiceJobRunPort: Send + Sync {
    // List JobRuns the user is allowed to view
    fn list_job_runs(
        &self,
        pagination: &PaginationQuery,
        user_id: UserId,
        job_id: &str,
    ) -> Result<PaginatedResponse<DisplayJobRun>, DomainError>;
}

/// Service port for job interactions.
pub trait ServiceJobPort: Send + Sync {
    fn create_job(&self, user_id: UserId, new_job: CreateJob) -> Result<DisplayJob, DomainError>;
    fn get_job_by_id(&self, user_id: UserId, job_id: &str) -> Result<DisplayJob, DomainError>;
    fn list_jobs(
        &self,
        pagination: &PaginationQuery,
        user_id: UserId,
    ) -> Result<PaginatedResponse<DisplayJob>, DomainError>;
    fn update_job(
        &self,
        user_id: UserId,
        job_id: &str,
        updated_job: UpdateRequestJob,
    ) -> Result<DisplayJob, DomainError>;
    fn delete_job(&self, user_id: UserId, job_id: &str) -> Result<(), DomainError>;
}
