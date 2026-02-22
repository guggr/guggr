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
}

/// Errors for everything Authentication related
#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("No Record found")]
    InvalidHashformat,
    #[error("Error while handling JWT: {0}")]
    JwtError(#[from] compact_jwt::error::JwtError),
    #[error("JWT expired")]
    JwtExpired,
    #[error("JTI claim not present")]
    JtiMissing,
    #[error("Storage: {0}")]
    Storage(#[from] StorageError),
    #[error("Auth Metadata has changed")]
    ChangedAuthMetadata,
}
