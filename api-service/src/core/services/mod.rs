pub mod auth;
pub mod groups;
pub mod users;

use std::sync::Arc;

use config::ApiServiceConfig;

use crate::core::ports::{repository::RepositoryPort, service::ServicePort};

pub struct Service {
    pub db: Arc<dyn RepositoryPort>,
    pub config: ApiServiceConfig,
}

impl Service {
    pub fn new(repo: Arc<dyn RepositoryPort>, config: ApiServiceConfig) -> Self {
        Self { db: repo, config }
    }
}

impl ServicePort for Service {}
