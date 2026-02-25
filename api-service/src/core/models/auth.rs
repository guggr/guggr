use chrono::{DateTime, TimeZone, Utc};
use database_client::models::RefreshToken;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::domain::errors::StorageError;

#[derive(Debug, Deserialize, ToSchema)]
/// request by the user to login
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
/// request by the user to logout
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
/// request by the user to get a new [`TokenResponse`]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric)]
/// returned to the user on login / token refresh
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric, Clone)]
/// used for handing down the `user_id` from the middleware to the requested
/// path
pub struct UserId(pub String);

#[derive(Debug, Serialize, PartialEq, Eq, LabelledGeneric, Default, Clone)]
pub struct AuthMetadata {
    pub ip_address: String,
    pub user_agent: String,
}

// DB DTO models below

#[derive(Debug, LabelledGeneric, Default)]
/// returned from the database on a user login
pub struct UserAuth {
    pub id: String,
    pub email: String,
    /// this is a argon2id hash
    pub password: String,
    pub jwt_secret: Vec<u8>,
}

#[derive(Debug, LabelledGeneric, Default)]
/// returned from the database on an auth check
pub struct UserAuthJwt {
    pub jwt_secret: Vec<u8>,
}

#[derive(Debug, Default)]
/// sent to the database for creating a new `RefreshToken` record
pub struct CreateRefreshToken {
    pub jti: String,
    pub user_id: String,
    pub expires_on: i64,
}

#[derive(Debug, LabelledGeneric, Default)]
/// returned from the database when an old refresh token was requested
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
