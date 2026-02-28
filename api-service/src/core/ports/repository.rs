use database_client::models::User;

use crate::core::domain::errors::DomainError;

/// RepositoryPort trait which requires all repository port traits.
pub trait RepositoryPort: RepositoryUserPort {}

pub trait RepositoryUserPort: Send + Sync {
    /// Inserts the user into the repository.
    fn create_user(&self, new_user: User) -> Result<User, DomainError>;
}
