use thiserror::Error;

/// Errors for the scheduler service
#[derive(Debug, Error)]
pub enum JobSchedulerError {
    /// Raised when the database connection fails
    #[error("database is currently unavailable: {0}")]
    DatabaseUnavailable(String),
    /// Raised when the `RabbitMQ` queue connection fails
    #[error("queue is currently unavailable: {0}")]
    QueueUnavailable(String),
    /// Raised when an internal error occurs that does not involve connection
    /// failures
    #[error("internal storage error: {0}")]
    Internal(String),
}
