use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth, UserAuthJwt},
        group::{CreateGroup, DisplayGroup, UpdateGroup},
        role::DisplayRole,
        user::{CreateUser, DisplayUser, UpdateUser},
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
    fn create(&self, new_value: N) -> Result<D, DomainError>;
    /// updates with the `UpdateStruct` and returns the new `DisplayStruct`
    /// either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn update(&self, user_id: Option<&str>, id: &str, update_value: U) -> Result<D, DomainError>;
    /// gets a `DisplayStruct` and returns it if it's found either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn get_by_id(&self, user_id: Option<&str>, id: &str) -> Result<Option<D>, DomainError>;
    /// deletes a record based on the `id` either if:
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), DomainError>;
    /// lists the `DisplayStruct` with the specified limit either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<D>, DomainError>;
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
    fn get_user_by_email(&self, email: &str) -> Result<UserAuth, DomainError>;
    /// get `UserAuthJwt` by a user's id
    fn get_user_jwt_secrets(&self, id: &str) -> Result<UserAuthJwt, DomainError>;
    /// create a new refresh token record in the database
    fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), DomainError>;
    /// get a `DisplayRefreshToken` by a jti claim
    fn get_refresh_token(&self, token: &str) -> Result<DisplayRefreshToken, DomainError>;
    /// delete a refresh token by a jti claim
    fn delete_refresh_token(&self, token: &str) -> Result<(), DomainError>;
    /// get all `DisplayRole` by a user's id
    fn get_roles_by_user(&self, id: &str) -> Result<Vec<DisplayRole>, DomainError>;
    /// check whether the user has the owner role
    fn is_owner(&self, id: &str) -> Result<bool, DomainError>;
}

/// Combines the [`RestrictedCrudOperations`], [`CrudOperations`],
/// [`AuthOperations`] and [`JobCrudOperations`] traits into a single trait
pub trait StoragePort: Send + Sync {
    fn user(
        &self,
    ) -> &(dyn RestrictedCrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync);
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
        pub user: MockStoreUser,
        pub group: MockStoreGroup,
        pub auth: MockStoreAuth,
    }
    pub struct MockStoreUser;
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
                user: MockStoreUser,
                group: MockStoreGroup,
                auth: MockStoreAuth,
            }
        }
    }

    impl StoragePort for MockStore {
        fn user(
            &self,
        ) -> &(dyn RestrictedCrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync)
        {
            &self.user
        }

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

    impl RestrictedCrudOperations<CreateUser, UpdateUser, DisplayUser> for MockStoreUser {
        fn create(&self, new_value: CreateUser) -> Result<DisplayUser, DomainError> {
            let user = DisplayUser {
                name: new_value.name,
                ..Default::default()
            };
            Ok(user)
        }
        fn update(
            &self,
            _user_id: Option<&str>,
            id: &str,
            update_value: UpdateUser,
        ) -> Result<DisplayUser, DomainError> {
            let user = DisplayUser {
                id: id.to_string(),
                email: "email".to_owned(),
                name: update_value.name.unwrap_or_default(),
            };
            Ok(user)
        }
        fn get_by_id(
            &self,
            _user_id: Option<&str>,
            id: &str,
        ) -> Result<Option<DisplayUser>, DomainError> {
            let user = DisplayUser {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(user))
        }
        fn delete(&self, _user_id: Option<&str>, _id: &str) -> Result<(), DomainError> {
            Ok(())
        }
        fn list(
            &self,
            _user_id: Option<&str>,
            limit: i64,
        ) -> Result<Vec<DisplayUser>, DomainError> {
            Ok(vec![DisplayUser::default(); limit as usize])
        }
    }

    impl RestrictedCrudOperations<CreateGroup, UpdateGroup, DisplayGroup> for MockStoreGroup {
        fn create(&self, new_value: CreateGroup) -> Result<DisplayGroup, DomainError> {
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
        ) -> Result<DisplayGroup, DomainError> {
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
        ) -> Result<Option<DisplayGroup>, DomainError> {
            let group = DisplayGroup {
                id: id.to_string(),
                ..Default::default()
            };
            Ok(Some(group))
        }
        fn delete(&self, _user_id: Option<&str>, _id: &str) -> Result<(), DomainError> {
            Ok(())
        }
        fn list(
            &self,
            _user_id: Option<&str>,
            limit: i64,
        ) -> Result<Vec<DisplayGroup>, DomainError> {
            Ok(vec![DisplayGroup::default(); limit as usize])
        }
    }

    impl AuthOperations for MockStoreAuth {
        fn get_user_by_email(&self, email: &str) -> Result<UserAuth, DomainError> {
            Ok(UserAuth {
                id: "cool-user".to_string(),
                email: email.to_string(),
                password: "cool-pass".to_string(),
                jwt_secret: vec![],
            })
        }
        fn get_user_jwt_secrets(&self, _id: &str) -> Result<UserAuthJwt, DomainError> {
            Ok(UserAuthJwt { jwt_secret: vec![] })
        }
        fn create_refresh_token(&self, _token: CreateRefreshToken) -> Result<(), DomainError> {
            Ok(())
        }
        fn get_refresh_token(&self, _jti: &str) -> Result<DisplayRefreshToken, DomainError> {
            Ok(DisplayRefreshToken::default())
        }
        fn delete_refresh_token(&self, _jti: &str) -> Result<(), DomainError> {
            Ok(())
        }
        fn get_roles_by_user(&self, _id: &str) -> Result<Vec<DisplayRole>, DomainError> {
            Ok(vec![])
        }
        fn is_owner(&self, _id: &str) -> Result<bool, DomainError> {
            Ok(true)
        }
    }
}
