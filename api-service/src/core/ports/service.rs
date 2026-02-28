use crate::core::{
    domain::errors::DomainError,
    models::user::{CreateUser, DisplayUser},
};

/// ServicePort trait which requires all service port traits.
pub trait ServicePort: ServiceUserPort {}

/// Service port including all user interactions.
pub trait ServiceUserPort: Send + Sync {
    /// Creates a new user
    fn create_user(&self, new_user: CreateUser) -> Result<DisplayUser, DomainError>;
}
