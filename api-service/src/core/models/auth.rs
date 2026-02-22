
use chrono::{DateTime, TimeZone, Utc};
use database_client::models::RefreshToken;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::domain::errors::StorageError;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct AuthMetadata {
    pub ip_address: String,
    pub user_agent: String,
}

// DB DTO models below

#[derive(Debug, LabelledGeneric)]
pub struct UserAuth {
    pub id: String,
    pub email: String,
    pub password: String,
    // TODO roles
}

#[derive(Debug)]
pub struct CreateRefreshToken {
    pub jti: String,
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub expires_on: i64,
}

#[derive(Debug)]
pub struct DisplayRefreshToken {
    pub user_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub expires_on: i64,
}

impl TryFrom<CreateRefreshToken> for RefreshToken {
    type Error = StorageError;
    fn try_from(value: CreateRefreshToken) -> Result<Self, Self::Error> {
        let exp = DateTime::<Utc>::from_timestamp_secs(value.expires_on)
            .ok_or(StorageError::TimestampConversion)?
            .naive_utc();
        Ok(Self {
            jti: value.jti,
            user_id: value.user_id,
            ip_address: value.ip_address,
            user_agent: value.user_agent,
            expires_on: exp,
        })
    }
}
impl From<RefreshToken> for DisplayRefreshToken {
    fn from(value: RefreshToken) -> Self {
        let ip = value.ip_address.to_string();
        let exp = Utc.from_utc_datetime(&value.expires_on).timestamp();
        Self {
            user_id: value.user_id,
            ip_address: ip,
            user_agent: value.user_agent,
            expires_on: exp,
        }
    }
}
