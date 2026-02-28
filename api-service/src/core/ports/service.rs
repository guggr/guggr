use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::{AuthenticatedResponse, LoginRequest, TokenRefreshRequest, TokenResponse, UserId},
        user::{CreateUser, DisplayUser},
    },
};

/// ServicePort trait which requires all service port traits.
pub trait ServicePort: ServiceUserPort + ServiceAuthPort {}

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
}
