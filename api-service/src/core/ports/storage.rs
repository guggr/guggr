use async_trait::async_trait;
use database_client::models::Group;

use crate::core::domain::errors::StorageError;

#[async_trait]
pub trait Crud<T> {
    async fn create(&self, new_value: T) -> Result<(), StorageError>;
    async fn update(&self, update_value: T) -> Result<(), StorageError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<T>, StorageError>;
    async fn delete(&self, id: &str) -> Result<(), StorageError>;
    async fn list(&self, limit: i64) -> Result<Vec<T>, StorageError>;
}

#[async_trait]
pub trait StoragePort: Send + Sync {
    type GroupCrud: Crud<Group>;
    fn group(&self) -> &Self::GroupCrud;
}
