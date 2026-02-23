use crate::core::{
    domain::errors::StorageError,
    models::{
        auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth},
        group::{CreateGroup, DisplayGroup, UpdateGroup},
        role::{CreateRole, DisplayRole, UpdateRole},
        user::{CreateUser, DisplayUser, UpdateUser},
    },
};

/// `N`: `NewStruct`, `U`: `UpdateStruct`, `D`: `DisplayStruct`
pub trait CrudOperations<N, U, D> {
    fn create(&self, new_value: N) -> Result<D, StorageError>;
    fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
    fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    fn delete(&self, id: &str) -> Result<(), StorageError>;
    fn list(&self, limit: i64) -> Result<Vec<D>, StorageError>;
}

pub trait AuthOperations {
    fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError>;
    fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError>;
    fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError>;
    fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError>;
}

pub trait StoragePort: Send + Sync {
    fn group(&self) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync);
    fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync);
    fn role(&self) -> &(dyn CrudOperations<CreateRole, UpdateRole, DisplayRole> + Send + Sync);
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync);
}

#[cfg(test)]
pub mod tests {

    use super::*;

    pub struct MockStore {
        pub group: MockStoreGroup,
        pub user: MockStoreUser,
        pub role: MockStoreRole,
        pub auth: MockStoreAuth,
    }
    pub struct MockStoreGroup;
    pub struct MockStoreUser;
    pub struct MockStoreRole;
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
                role: MockStoreRole,
                auth: MockStoreAuth,
            }
        }
    }

    impl StoragePort for MockStore {
        fn group(
            &self,
        ) -> &(dyn CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync) {
            &self.group
        }
        fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync) {
            &self.user
        }
        fn role(&self) -> &(dyn CrudOperations<CreateRole, UpdateRole, DisplayRole> + Send + Sync) {
            &self.role
        }
        fn auth(&self) -> &(dyn AuthOperations + Send + Sync) {
            &self.auth
        }
    }

    impl CrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for MockStoreGroup {
        fn create(&self, new_value: CreateGroup) -> Result<DisplayGroup, StorageError> {
            let user = DisplayGroup {
                name: new_value.name,
                ..Default::default()
            };
            Ok(user)
        }
        fn update(
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
        fn get_by_id(&self, id: &str) -> Result<Option<DisplayGroup>, StorageError> {
            let group = DisplayGroup {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(group))
        }
        fn delete(&self, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        fn list(&self, limit: i64) -> Result<Vec<DisplayGroup>, StorageError> {
            Ok(vec![DisplayGroup::default(); limit as usize])
        }
    }

    impl CrudOperations<CreateUser, UpdateUser, DisplayUser> for MockStoreUser {
        fn create(&self, new_value: CreateUser) -> Result<DisplayUser, StorageError> {
            let user = DisplayUser {
                name: new_value.name,
                email: new_value.email,
                ..Default::default()
            };
            Ok(user)
        }
        fn update(&self, id: &str, update_value: UpdateUser) -> Result<DisplayUser, StorageError> {
            let user = DisplayUser {
                id: id.to_string(),
                name: update_value.name.unwrap_or_default(),
                email: update_value.email.unwrap_or_default(),
            };
            Ok(user)
        }
        fn get_by_id(&self, id: &str) -> Result<Option<DisplayUser>, StorageError> {
            let user = DisplayUser {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(user))
        }
        fn delete(&self, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        fn list(&self, limit: i64) -> Result<Vec<DisplayUser>, StorageError> {
            Ok(vec![DisplayUser::default(); limit as usize])
        }
    }
    impl CrudOperations<CreateRole, UpdateRole, DisplayRole> for MockStoreRole {
        fn create(&self, new_value: CreateRole) -> Result<DisplayRole, StorageError> {
            let user = DisplayRole {
                name: new_value.name,
                ..Default::default()
            };
            Ok(user)
        }
        fn update(&self, id: &str, update_value: UpdateRole) -> Result<DisplayRole, StorageError> {
            let user = DisplayRole {
                id: id.to_string(),
                name: update_value.name.unwrap_or_default(),
            };
            Ok(user)
        }
        fn get_by_id(&self, id: &str) -> Result<Option<DisplayRole>, StorageError> {
            let user = DisplayRole {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(user))
        }
        fn delete(&self, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        fn list(&self, limit: i64) -> Result<Vec<DisplayRole>, StorageError> {
            Ok(vec![DisplayRole::default(); limit as usize])
        }
    }
    impl AuthOperations for MockStoreAuth {
        fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError> {
            Ok(UserAuth {
                id: "cool-user".to_string(),
                email: email.to_string(),
                password: "cool-pass".to_string(),
            })
        }
        fn create_refresh_token(&self, _token: CreateRefreshToken) -> Result<(), StorageError> {
            Ok(())
        }
        fn get_refresh_token(&self, _jti: &str) -> Result<DisplayRefreshToken, StorageError> {
            Ok(DisplayRefreshToken::default())
        }
        fn delete_refresh_token(&self, _jti: &str) -> Result<(), StorageError> {
            Ok(())
        }
    }
}
