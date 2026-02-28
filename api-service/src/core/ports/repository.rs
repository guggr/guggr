use database_client::models::User;

use crate::core::domain::errors::DomainError;

/// RepositoryPort trait which requires all repository port traits.
pub trait RepositoryPort: RepositoryUserPort {}

pub trait RepositoryUserPort: Send + Sync {
    /// Inserts the user into the repository.
    fn create_user(&self, new_user: User) -> Result<User, DomainError>;

    /// Get the user from the repository by ID.
    fn get_user(&self, id: &str) -> Result<User, DomainError>;

    /// Get the user from the repository by ID.
    fn get_user_by_email(&self, email: &str) -> Result<User, DomainError>;
}
