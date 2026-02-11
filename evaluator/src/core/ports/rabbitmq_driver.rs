use async_trait::async_trait;

use crate::core::domain::errors::JobEvaluatorError;

#[async_trait]
pub trait RabbitMQDriverPort: Send + Sync {
    async fn setup(&self) -> Result<(), JobEvaluatorError>;
    async fn start(&self) -> Result<(), JobEvaluatorError>;
}
