use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::{AuthenticatedResponse, LoginRequest},
        user::{CreateUser, DisplayUser},
    },
};

/// ServicePort trait which requires all service port traits.
pub trait ServicePort: ServiceUserPort + ServiceAuthPort {}

/// Service port including all user interactions.
pub trait ServiceUserPort: Send + Sync {
    /// Creates a new user
    fn create_user(&self, new_user: CreateUser) -> Result<DisplayUser, DomainError>;
}

/// Service port for auth interactions.
pub trait ServiceAuthPort: Send + Sync {
    fn login(&self, login_req: LoginRequest) -> Result<AuthenticatedResponse, DomainError>;
}
