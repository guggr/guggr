use thiserror::Error;

#[derive(Debug, Error)]
pub enum JobEvaluatorError {
    #[error("Storage is currently unavailable")]
    Unavailable,
    #[error("Internal storage error: {0}")]
    Internal(String),
}
