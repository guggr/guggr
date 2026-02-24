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

/// `N`: `NewStruct`, `U`: `UpdateStruct`, `D`: `DisplayStruct`
pub trait CrudOperations<N, U, D> {
    fn create(&self, new_value: N) -> Result<D, StorageError>;
    fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
    fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    fn delete(&self, id: &str) -> Result<(), StorageError>;
    fn list(&self, limit: i64) -> Result<Vec<D>, StorageError>;
}

pub trait RestrictedCrudOperations<N, U, D> {
    fn create(&self, new_value: N) -> Result<D, StorageError>;
    fn update(&self, user_id: Option<&str>, id: &str, update_value: U) -> Result<D, StorageError>;
    fn get_by_id(&self, user_id: Option<&str>, id: &str) -> Result<Option<D>, StorageError>;
    fn delete(&self, user_id: Option<&str>, id: &str) -> Result<(), StorageError>;
    fn list(&self, user_id: Option<&str>, limit: i64) -> Result<Vec<D>, StorageError>;
}

pub trait JobCrudOperations {
    fn create(&self, new_value: CreateJob) -> Result<DisplayJob, StorageError>;
    fn update(&self, id: &str, update_value: UpdateJob) -> Result<DisplayJob, StorageError>;
    fn get_by_id(&self, id: &str) -> Result<Option<DisplayJob>, StorageError>;
    fn delete(&self, id: &str) -> Result<(), StorageError>;
    fn list(&self, limit: i64) -> Result<Vec<DisplayJob>, StorageError>;

    fn run(&self) -> &(dyn JobRunCrudOperations + Send + Sync);
}

pub trait JobRunCrudOperations {
    fn get_by_job_id(&self, job_id: &str) -> Result<Option<DisplayJobRun>, StorageError>;
    fn list_by_job_id(&self, job_id: &str, limit: i64) -> Result<Vec<DisplayJobRun>, StorageError>;
}

pub trait JobDetailOperations<U, D> {
    fn create(&self, new_value: D) -> Result<D, StorageError>;
    fn get_by_id(&self, id: &str) -> Result<Option<D>, StorageError>;
    fn update(&self, id: &str, update_value: U) -> Result<D, StorageError>;
}

pub trait AuthOperations {
    fn get_user_by_email(&self, email: &str) -> Result<UserAuth, StorageError>;
    fn get_user_jwt_secrets(&self, id: &str) -> Result<UserAuthJwt, StorageError>;
    fn create_refresh_token(&self, token: CreateRefreshToken) -> Result<(), StorageError>;
    fn get_refresh_token(&self, jti: &str) -> Result<DisplayRefreshToken, StorageError>;
    fn delete_refresh_token(&self, jti: &str) -> Result<(), StorageError>;
    fn get_roles_by_user(&self, id: &str) -> Result<Vec<DisplayRole>, StorageError>;
    fn is_owner(&self, id: &str) -> Result<bool, StorageError>;
}

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
        fn create(&self, _new_value: CreateJob) -> Result<DisplayJob, StorageError> {
            todo!()
        }
        fn delete(&self, _id: &str) -> Result<(), StorageError> {
            todo!()
        }
        fn get_by_id(&self, _id: &str) -> Result<Option<DisplayJob>, StorageError> {
            todo!()
        }
        fn list(&self, _limit: i64) -> Result<Vec<DisplayJob>, StorageError> {
            todo!()
        }
        fn run(&self) -> &(dyn JobRunCrudOperations + Send + Sync) {
            &self.run
        }
        fn update(&self, _id: &str, _update_value: UpdateJob) -> Result<DisplayJob, StorageError> {
            todo!()
        }
    }
    impl JobRunCrudOperations for MockStoreJobRun {
        fn get_by_job_id(&self, _job_id: &str) -> Result<Option<DisplayJobRun>, StorageError> {
            todo!()
        }
        fn list_by_job_id(
            &self,
            _job_id: &str,
            _limit: i64,
        ) -> Result<Vec<DisplayJobRun>, StorageError> {
            todo!()
        }
    }
}
