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
}
