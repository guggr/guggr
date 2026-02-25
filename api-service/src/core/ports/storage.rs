use crate::core::{
    domain::errors::StorageError,
    models::{
        auth::{CreateRefreshToken, DisplayRefreshToken, UserAuth, UserAuthJwt},
        group::{CreateGroup, DisplayGroup, UpdateGroup},
        job::{CreateJob, DisplayJob, UpdateJob, run::DisplayJobRun},
        role::{CreateRole, DisplayRole, UpdateRole},
        user::{CreateUser, DisplayUser, UpdateUser},
    },
};

/// Adds generitc CRUD Operations for the following DTO models:
/// `N`: `NewStruct`, `U`: `UpdateStruct`, `D`: `DisplayStruct`
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
pub trait CrudOperations<N, U, D> {
    /// creates the `NewStruct` and returns the `DisplayStruct`
    fn create(&self, new_value: N) -> Result<D, StorageError>;
    /// updates with UpdateStruct and returns the new `DisplayStruct`
    fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
    /// gets a `DisplayStruct` and returns it if it's found
    fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    /// deletes a record based on the `id`
    fn delete(&self, id: &str) -> Result<(), StorageError>;
    /// lists the `DisplayStruct` with the specified limit
    fn list(&self, limit: i64) -> Result<Vec<D>, StorageError>;
}

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

/// Adds restricted [`database_client::models::Job`] CRUD Operations:
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
/// - the user does not have permissions
pub trait JobCrudOperations {
    /// creates the [`CreateJob`] and returns the new [`DisplayJob`] either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   will own that record
    /// - no `user_id` is supplied
    fn create(
        &self,
        user_id: Option<&str>,
        new_value: CreateJob,
    ) -> Result<DisplayJob, StorageError>;
    /// updates with the [`UpdateJob`] and returns the new [`DisplayJob`] either
    /// if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn update(
        &self,
        user_id: Option<&str>,
        id: &str,
        update_value: UpdateJob,
    ) -> Result<DisplayJob, StorageError>;
    /// gets a [`DisplayJob`] and returns it if it's found either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn get_by_id(
        &self,
        user_id: Option<&str>,
        id: &str,
    ) -> Result<Option<DisplayJob>, StorageError>;
    /// deletes a record based on the `id` either if:
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), StorageError>;
    /// lists the [`DisplayJob`] with the specified limit either if
    /// - a `user_id` is supplied, and the `user_id` is part of the group that
    ///   owns that record
    /// - no `user_id` is supplied
    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<DisplayJob>, StorageError>;

    fn run(&self) -> &(dyn JobRunCrudOperations + Send + Sync);
}

/// Adds restricted [`database_client::models::JobRun`] get/list Operations:
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
/// - the user does not have permissions
pub trait JobRunCrudOperations {
    fn get_by_job_id(
        &self,
        user_id: Option<&str>,
        job_id: &str,
    ) -> Result<Option<DisplayJobRun>, StorageError>;
    fn list_by_job_id(
        &self,
        user_id: Option<&str>,
        job_id: &str,
        limit: i64,
    ) -> Result<Vec<DisplayJobRun>, StorageError>;
}

/// Adds `JobDetails` CRU Operations for the following DTO models:
/// `U`: `UpdateStruct`, `D`: `DisplayStruct`
/// (delete is handled by the database due to the FK CASCADE ON DELETE
/// constraint)
///
/// # Errors
/// Will return [`StorageError`] if either
/// - the database is unavailable
/// - no record was found
/// - the timestamp could not be converted
/// - an internal error occurred
/// - the user does not have permissions
pub trait JobDetailOperations<U, D> {
    fn create(&self, new_value: D) -> Result<D, StorageError>;
    fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
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
    fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError>;
    /// delete a refresh token by a jti claim
    fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError>;
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
    fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync);
    fn role(&self) -> &(dyn CrudOperations<CreateRole, UpdateRole, DisplayRole> + Send + Sync);
    fn auth(&self) -> &(dyn AuthOperations + Send + Sync);
    fn job(&self) -> &(dyn JobCrudOperations + Send + Sync);
}

#[cfg(test)]
pub mod tests {

    use std::vec;

    use super::*;

    pub struct MockStore {
        pub group: MockStoreGroup,
        pub user: MockStoreUser,
        pub role: MockStoreRole,
        pub auth: MockStoreAuth,
        pub job: MockStoreJob,
    }
    pub struct MockStoreGroup;
    pub struct MockStoreUser;
    pub struct MockStoreRole;
    pub struct MockStoreAuth;
    pub struct MockStoreJob {
        pub run: MockStoreJobRun,
    }
    pub struct MockStoreJobRun;

    impl Default for MockStore {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Default for MockStoreJob {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockStoreJob {
        pub fn new() -> Self {
            Self {
                run: MockStoreJobRun,
            }
        }
    }

    impl MockStore {
        pub fn new() -> Self {
            Self {
                group: MockStoreGroup,
                user: MockStoreUser,
                role: MockStoreRole,
                auth: MockStoreAuth,
                job: MockStoreJob::new(),
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
        fn user(&self) -> &(dyn CrudOperations<CreateUser, UpdateUser, DisplayUser> + Send + Sync) {
            &self.user
        }
        fn role(&self) -> &(dyn CrudOperations<CreateRole, UpdateRole, DisplayRole> + Send + Sync) {
            &self.role
        }
        fn auth(&self) -> &(dyn AuthOperations + Send + Sync) {
            &self.auth
        }
        fn job(&self) -> &(dyn JobCrudOperations + Send + Sync) {
            &self.job
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
    impl JobCrudOperations for MockStoreJob {
        fn create(
            &self,
            _user_id: Option<&str>,
            _new_value: CreateJob,
        ) -> Result<DisplayJob, StorageError> {
            todo!()
        }
        fn delete(&self, _user_id: Option<&str>, _id: &str) -> Result<(), StorageError> {
            todo!()
        }
        fn get_by_id(
            &self,
            _user_id: Option<&str>,
            _id: &str,
        ) -> Result<Option<DisplayJob>, StorageError> {
            todo!()
        }
        fn list(
            &self,
            _user_id: Option<&str>,
            _limit: i64,
        ) -> Result<Vec<DisplayJob>, StorageError> {
            todo!()
        }
        fn run(&self) -> &(dyn JobRunCrudOperations + Send + Sync) {
            &self.run
        }
        fn update(
            &self,
            _user_id: Option<&str>,
            _id: &str,
            _update_value: UpdateJob,
        ) -> Result<DisplayJob, StorageError> {
            todo!()
        }
    }
    impl JobRunCrudOperations for MockStoreJobRun {
        fn get_by_job_id(
            &self,
            _user_id: Option<&str>,
            _job_id: &str,
        ) -> Result<Option<DisplayJobRun>, StorageError> {
            todo!()
        }
        fn list_by_job_id(
            &self,
            _user_id: Option<&str>,
            _job_id: &str,
            _limit: i64,
        ) -> Result<Vec<DisplayJobRun>, StorageError> {
            todo!()
        }
    }
}
