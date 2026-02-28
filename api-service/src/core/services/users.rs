use crate::core::{
    domain::errors::DomainError,
    models::user::{CreateUser, DisplayUser},
    ports::service::ServiceUserPort,
    services::Service,
};

impl ServiceUserPort for Service {
    fn create_user(&self, new_user: CreateUser) -> Result<DisplayUser, DomainError> {
        self.db.user().create(new_user)
    }
}
