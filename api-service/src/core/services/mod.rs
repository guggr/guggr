pub mod users;

use std::sync::Arc;

use crate::core::ports::{service::ServicePort, storage::StoragePort};

pub struct Service {
    pub db: Arc<dyn StoragePort>,
}

impl Service {
    pub fn new(storage: Arc<dyn StoragePort>) -> Self {
        Self { db: storage }
    }
}

impl ServicePort for Service {}
