use async_trait::async_trait;

use crate::core::{
    domain::errors::StorageError,
    models::{
        auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth},
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
    async fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError>;
    async fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError>;
    async fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError>;
}

#[async_trait]
pub trait StoragePort: Send + Sync {
    fn group(&self) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync);
    fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync);
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync);
}

#[cfg(test)]
pub mod tests {

    use super::*;

    pub struct MockStore {
        pub group: MockStoreGroup,
        pub user: MockStoreUser,
        pub auth: MockStoreAuth,
    }
    pub struct MockStoreGroup;
    pub struct MockStoreUser;
    pub struct MockStoreAuth;

    impl Default for MockStore {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockStore {
        pub fn new() -> Self {
            Self {
                group: MockStoreGroup,
                user: MockStoreUser,
                auth: MockStoreAuth,
            }
        }
    }

    #[async_trait]
    impl StoragePort for MockStore {
        fn group(
            &self,
        ) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync) {
            &self.group
        }
        fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync) {
            &self.user
        }

        fn auth(&self) -> &(dyn AuthOperations + Send + Sync) {
            &self.auth
        }
    }

    #[async_trait]
    impl CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for MockStoreGroup {
        async fn create(&self, new_value: CreateGroup) -> Result<DisplayGroup, StorageError> {
            let user = DisplayGroup {
                name: new_value.name,
                ..Default::default()
            };
            Ok(user)
        }
        async fn update(
            &self,
            id: &str,
            update_value: UpdateGroup,
        ) -> Result<DisplayGroup, StorageError> {
            let group = DisplayGroup {
                id: id.to_string(),
                name: update_value.name.unwrap_or_default(),
            };
            Ok(group)
        }
        async fn get_by_id(&self, id: &str) -> Result<Option<DisplayGroup>, StorageError> {
            let group = DisplayGroup {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(group))
        }
        async fn delete(&self, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        async fn list(&self, limit: i64) -> Result<Vec<DisplayGroup>, StorageError> {
            Ok(vec![DisplayGroup::default(); limit as usize])
        }
    }

    #[async_trait]
    impl CrudOperations<CreateUser, UpdateUser, DisplayUser> for MockStoreUser {
        async fn create(&self, new_value: CreateUser) -> Result<DisplayUser, StorageError> {
            let user = DisplayUser {
                name: new_value.name,
                email: new_value.email,
                ..Default::default()
            };
            Ok(user)
        }
        async fn update(
            &self,
            id: &str,
            update_value: UpdateUser,
        ) -> Result<DisplayUser, StorageError> {
            let user = DisplayUser {
                id: id.to_string(),
                name: update_value.name.unwrap_or_default(),
                email: update_value.email.unwrap_or_default(),
            };
            Ok(user)
        }
        async fn get_by_id(&self, id: &str) -> Result<Option<DisplayUser>, StorageError> {
            let user = DisplayUser {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(user))
        }
        async fn delete(&self, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        async fn list(&self, limit: i64) -> Result<Vec<DisplayUser>, StorageError> {
            Ok(vec![DisplayUser::default(); limit as usize])
        }
    }

    #[async_trait]
    impl AuthOperations for MockStoreAuth {
        async fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError> {
            Ok(UserAuth {
                id: "cool-user".to_string(),
                email: email.to_string(),
                password: "cool-pass".to_string(),
            })
        }
        async fn create_refresh_token(
            &self,
            _token: CreateRefreshToken,
        ) -> Result<(), StorageError> {
            Ok(())
        }
        async fn get_refresh_token(&self, _jti: &str) -> Result<DisplayRefreshToken, StorageError> {
            Ok(DisplayRefreshToken::default())
        }
        async fn delete_refresh_token(&self, _jti: &str) -> Result<(), StorageError> {
            Ok(())
        }
    }
}
