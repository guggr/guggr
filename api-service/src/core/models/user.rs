use database_client::{models::User, schema::user};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::domain::auth_helper::{generate_user_jwt_secret, hash_password};

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    ToSchema,
    LabelledGeneric,
    Validate,
    Default,
)]
/// Struct to create a new user
pub struct CreateUser {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub email: String,

    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub password: String,

    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: String,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    Serialize,
    Deserialize,
    ToSchema,
    AsChangeset,
    LabelledGeneric,
    Validate,
    Default,
)]
#[diesel(table_name = user)]
/// Struct to update a user
/// TODO handle password/email change
pub struct UpdateUser {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: Option<String>,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
/// Returned user
pub struct DisplayUser {
    pub id: String,
    pub email: String,
    pub name: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        Self {
            id: nanoid::nanoid!(),
            email: value.email,
            password: hash_password(&value.password),
            name: value.name,
            jwt_secret: generate_user_jwt_secret(),
        }
    }
}
