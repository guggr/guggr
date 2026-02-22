use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub access_token: String,
}

#[derive(Debug, LabelledGeneric)]
pub struct UserAuth {
    pub id: String,
    pub email: String,
    pub password: String,
    // TODO roles
}
