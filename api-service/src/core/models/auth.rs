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
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric, Clone)]
pub struct UserId(pub String);

#[derive(Debug, Serialize, PartialEq, Eq, LabelledGeneric, Default, Clone)]
pub struct AuthMetadata {
    pub ip_address: String,
    pub user_agent: String,
}

// DB DTO models below

#[derive(Debug, LabelledGeneric, Default)]
pub struct UserAuth {
    pub id: String,
    pub email: String,
    pub password: String,
    pub jwt_secret: Vec<u8>,
}

#[derive(Debug, LabelledGeneric, Default)]
pub struct UserAuthJwt {
    pub jwt_secret: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct CreateRefreshToken {
    pub jti: String,
    pub user_id: String,
    pub expires_on: i64,
}

#[derive(Debug, LabelledGeneric, Default)]
pub struct DisplayRefreshToken {
    pub user_id: String,
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
            expires_on: exp,
        })
    }
}
impl From<RefreshToken> for DisplayRefreshToken {
    fn from(value: RefreshToken) -> Self {
        let exp = Utc.from_utc_datetime(&value.expires_on).timestamp();
        Self {
            user_id: value.user_id,
            expires_on: exp,
        }
    }
}
