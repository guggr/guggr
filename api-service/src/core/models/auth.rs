use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::models::user::DisplayUser;

#[derive(Debug, Deserialize, ToSchema)]
/// Data used to authenticate.
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
/// Data used to logout.
pub struct LogoutRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, ToSchema)]
/// Data used to refresh the access and refresh tokens.
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric)]
/// Data sent upon login.
pub struct AuthenticatedResponse {
    pub user: DisplayUser,
    pub auth: TokenResponse,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric)]
/// Authentication tokens as received after successful authentication.
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema, LabelledGeneric, Clone)]
/// Passing `user_id` from the middleware to endpoint handlers path
pub struct UserId(pub String);
