use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobSchedulerError {
    #[error("Database is currently unavailable")]
    DatabaseUnavailable,
    #[error("Queue is currently unavailable")]
    QueueUnavailable,
    #[error("Internal storage error: {0}")]
    Internal(String),
}
