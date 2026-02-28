use database_client::schema::user;
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
