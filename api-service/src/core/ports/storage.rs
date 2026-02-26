use crate::core::{
    domain::errors::StorageError,
    models::{
        auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth, UserAuthJwt},
        group::{CreateGroup, DisplayGroup, UpdateGroup},
        role::DisplayRole,
    },
};

/// Adds generitc restricted CRUD Operations for the following DTO models:
/// `N`: `NewStruct`, `U`: `UpdateStruct`, `D`: `DisplayStruct`
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
/// - the user does not have permissions
pub trait RestrictedCrudOperations<N, U, D> {
    /// creates the `NewStruct` and returns the `DisplayStruct`
    fn create(&self, new_value: N) -> Result<D, StorageError>;
    /// updates with the `UpdateStruct` and returns the new `DisplayStruct`
    /// either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn update(&self, user_id: Option<&str>, id: &str, update_value: U) -> Result<D, StorageError>;
    /// gets a `DisplayStruct` and returns it if it's found either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn get_by_id(&self, user_id: Option<&str>, id: &str) -> Result<Option<D>, StorageError>;
    /// deletes a record based on the `id` either if:
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), StorageError>;
    /// lists the `DisplayStruct` with the specified limit either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<D>, StorageError>;
}

/// Adds Auth relevant Operations:
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
pub trait AuthOperations {
    /// get `UserAuth` by a user's email
    fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError>;
    /// get `UserAuthJwt` by a user's id
    fn get_user_jwt_secrets(&self, id: &str) -> Result<UserAuthJwt, StorageError>;
    /// create a new refresh token record in the database
    fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError>;
    /// get a `DisplayRefreshToken` by a jti claim
    fn get_refresh_token(&self, token: &str) -> Result<DisplayRefreshToken, StorageError>;
    /// delete a refresh token by a jti claim
    fn delete_refresh_token(&self, token: &str) -> Result<(), StorageError>;
    /// get all `DisplayRole` by a user's id
    fn get_roles_by_user(&self, id: &str) -> Result<Vec<DisplayRole>, StorageError>;
    /// check whether the user has the owner role
    fn is_owner(&self, id: &str) -> Result<bool, StorageError>;
}

/// Combines the [`RestrictedCrudOperations`], [`CrudOperations`],
/// [`AuthOperations`] and [`JobCrudOperations`] traits into a single trait
pub trait StoragePort: Send + Sync {
    fn group(
        &self,
    ) -> &(dyn RestrictedCrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync);
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync);
}

#[cfg(test)]
pub mod tests {

    use std::vec;

    use super::*;

    pub struct MockStore {
        pub group: MockStoreGroup,
        pub auth: MockStoreAuth,
    }
    pub struct MockStoreGroup;
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
                auth: MockStoreAuth,
            }
        }
    }

    impl StoragePort for MockStore {
        fn group(
            &self,
        ) -> &(dyn RestrictedCrudOperations<CreateGroup, UpdateGroup, DisplayGroup> + Send + Sync)
        {
            &self.group
        }

        fn auth(&self) -> &(dyn AuthOperations + Send + Sync) {
            &self.auth
        }
    }

    impl RestrictedCrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for MockStoreGroup {
        fn create(&self, new_value: CreateGroup) -> Result<DisplayGroup, StorageError> {
            let user = DisplayGroup {
                name: new_value.name,
                ..Default::default()
            };
            Ok(user)
        }
        fn update(
            &self,
            _user_id: Option<&str>,
            id: &str,
            update_value: UpdateGroup,
        ) -> Result<DisplayGroup, StorageError> {
            let group = DisplayGroup {
                id: id.to_string(),
                name: update_value.name.unwrap_or_default(),
            };
            Ok(group)
        }
        fn get_by_id(
            &self,
            _user_id: Option<&str>,
            id: &str,
        ) -> Result<Option<DisplayGroup>, StorageError> {
            let group = DisplayGroup {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(group))
        }
        fn delete(&self, _user_id: Option<&str>, _id: &str) -> Result<(), StorageError> {
            Ok(())
        }
        fn list(
            &self,
            _user_id: Option<&str>,
            limit: i64,
        ) -> Result<Vec<DisplayGroup>, StorageError> {
            Ok(vec![DisplayGroup::default(); limit as usize])
        }
    }

    impl AuthOperations for MockStoreAuth {
        fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError> {
            Ok(UserAuth {
                id: "cool-user".to_string(),
                email: email.to_string(),
                password: "cool-pass".to_string(),
                jwt_secret: vec![],
            })
        }
        fn get_user_jwt_secrets(&self, _id: &str) -> Result<UserAuthJwt, StorageError> {
            Ok(UserAuthJwt { jwt_secret: vec![] })
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
        fn get_roles_by_user(&self, _id: &str) -> Result<Vec<DisplayRole>, StorageError> {
            Ok(vec![])
        }
        fn is_owner(&self, _id: &str) -> Result<bool, StorageError> {
            Ok(true)
        }
    }
}
