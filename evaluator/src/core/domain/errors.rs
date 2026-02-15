use thiserror::Error;

/// Errors for the `EvalService`
#[derive(Debug, Error, PartialEq, Eq)]
pub enum JobEvaluatorError {
    /// Raised when either the database or rabbitmq is unavailable
    #[error("Database/RabbitMQ is currently unavailable: {0}")]
    Unavailable(String),
    /// Raised when an issue occurred while processing a job
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Errors for the typer mappers
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TypeMapperError {
    /// Raised when the Timestamp could not be converted
    #[error("Could not convert the supplied timestamp: {0}")]
    Timestamp(String),
    /// Raised when the IP Address could not be converted
    #[error("Could not convert the supplied IP Address: {0}")]
    IpAddress(String),
    /// Raised when the Latency could not be converted
    #[error("Could not convert the supplied Latency: {0}")]
    Latency(String),
}
