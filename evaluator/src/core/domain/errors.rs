use thiserror::Error;

/// Errors for [`JobEvaluatorError`]
///
/// - [`JobEvaluatorError::Unavailable`] is raised when either the database or
///   rabbitmq is unavailable
/// - [`JobEvaluatorError::Internal`] is raised when an issue occurred while
///   processing a job
#[derive(Debug, Error)]
pub enum JobEvaluatorError {
    #[error("Rabbitmq or postgres is currently unavailable")]
    Unavailable,
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Error)]
pub enum TypeMapperError {
    #[error("Could not convert the supplied timestamp: {0}")]
    Timestamp(String),
    #[error("Could not convert the supplied IP Address: {0}")]
    IpAddress(String),
    #[error("Could not convert the supplied Latency: {0}")]
    Latency(String),
}
