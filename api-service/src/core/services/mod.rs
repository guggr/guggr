pub mod users;

use std::sync::Arc;

use crate::core::ports::{repository::RepositoryPort, service::ServicePort};

pub struct Service {
    pub db: Arc<dyn RepositoryPort>,
}

impl Service {
    pub fn new(repo: Arc<dyn RepositoryPort>) -> Self {
        Self { db: repo }
    }
}

impl ServicePort for Service {}
