use frunk::LabelledGeneric;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, LabelledGeneric)]
pub struct UserAuth {
    pub id: String,
    pub email: String,
    pub password: String,
    // TODO roles
}
