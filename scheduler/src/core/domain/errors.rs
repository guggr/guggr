use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobSchedulerError {
    #[error("Storage is currently unavailable")]
    Unavailable,
    #[error("Internal storage error: {0}")]
    Internal(String),
}
