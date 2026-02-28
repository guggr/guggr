use database_client::models::User;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::{
        auth_helper::{generate_user_jwt_secret, hash_password},
        errors::DomainError,
    },
    models::user::{CreateUser, DisplayUser},
    ports::service::ServiceUserPort,
    services::Service,
};

impl ServiceUserPort for Service {
    fn create_user(&self, new_user: CreateUser) -> Result<DisplayUser, DomainError> {
        // TODO validate email

        let user = User {
            id: nanoid::nanoid!(),
            email: new_user.email,
            password: hash_password(&new_user.password),
            name: new_user.name,
            jwt_secret: generate_user_jwt_secret(),
        };

        let db_user = self.db.create_user(user)?;

        Ok(db_user.transmogrify())
    }
}
