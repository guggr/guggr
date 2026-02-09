use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobRepositoryError {
    #[error("Storage is currently unavailable")]
    Unavailable,
    #[error("Internal storage error: {0}")]
    Internal(String),
}
