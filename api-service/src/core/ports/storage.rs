use async_trait::async_trait;

use crate::core::{
    domain::errors::StorageError,
    models::{
        auth::UserAuth,
        group::{CreateGroup, DisplayGroup, UpdateGroup},
        user::{CreateUser, DisplayUser, UpdateUser},
    },
};

/// `N`: `NewStruct`, `U`: `UpdateStruct`, `D`: `DisplayStruct`
#[async_trait]
pub trait CrudOperations<N, U, D> {
    async fn create(&self, new_value: N) -> Result<D, StorageError>;
    async fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
    async fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    async fn delete(&self, id: &str) -> Result<(), StorageError>;
    async fn list(&self, limit: i64) -> Result<Vec<D>, StorageError>;
}

#[async_trait]
pub trait AuthOperations {
    async fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError>;
}

#[async_trait]
pub trait StoragePort: Send + Sync {
    fn group(&self) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync);
    fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync);
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync);
}
