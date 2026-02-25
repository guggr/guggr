use database_client::{models::Role, schema::role};
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
/// sent to the database for creating a new role
pub struct CreateRole {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
/// returned from the database when displaying a role
pub struct DisplayRole {
    pub id: String,
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
#[diesel(table_name = role)]
/// sent to the database when updating a role
pub struct UpdateRole {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: Option<String>,
}

impl From<CreateRole> for Role {
    fn from(value: CreateRole) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
        }
    }
}
