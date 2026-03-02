use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::{
            AuthenticatedResponse, LoginRequest, LogoutRequest, TokenRefreshRequest, TokenResponse,
            UserId,
        },
        group::{CreateGroup, DisplayGroup},
        user::{CreateUser, DisplayUser},
    },
};

/// ServicePort trait which requires all service port traits.
pub trait ServicePort: ServiceUserPort + ServiceAuthPort + ServiceGroupPort {}

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
    /// List groups by the supplied user ID
    fn list_groups_by_user(&self, user_id: UserId) -> Result<Vec<DisplayGroup>, DomainError>;
}
