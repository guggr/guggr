use thiserror::Error;

/// Errors for `StoragePort`
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StorageError {
    #[error("Storage is currently unavailable: {0}")]
    Unavailable(String),
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("No Record found")]
    NotFound,
    #[error("Timestamp conversion error")]
    TimestampConversion,
    #[error("User not authorized")]
    Unauthorized,
}

/// Errors for everything Authentication related
#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("No Record found")]
    InvalidHashformat,
    #[error("Error while handling JWT: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Storage: {0}")]
    Storage(#[from] StorageError),
    #[error("User not authorized")]
    Unauthorized,
    #[error("Argon2 error")]
    Argon2,
}
