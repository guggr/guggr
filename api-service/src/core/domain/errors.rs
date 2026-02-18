use thiserror::Error;

/// Errors for `StoragePort`
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StorageError {
    #[error("Storage is currently unavailable: {0}")]
    Unavailable(String),
}
