use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobSchedulerError {
    #[error("Database is currently unavailable: {0}")]
    DatabaseUnavailable(String),
    #[error("Queue is currently unavailable: {0}")]
    QueueUnavailable(String),
    #[error("Internal storage error: {0}")]
    Internal(String),
}
