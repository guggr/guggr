use database_client::models::{RefreshToken, User};

use crate::core::domain::errors::DomainError;

/// RepositoryPort trait which requires all repository port traits.
pub trait RepositoryPort: RepositoryUserPort + RepositoryRefreshTokenPort {}

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
