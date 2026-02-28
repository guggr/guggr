pub mod auth;
pub mod users;

use crate::core::ports::{service::ServicePort, storage::StoragePort};
use std::sync::Arc;

pub struct Service {
    pub db: Arc<dyn StoragePort>,
}

impl Service {
    pub fn new(storage: Arc<dyn StoragePort>) -> Self {
        Self { db: storage }
    }
}

impl ServicePort for Service {}
