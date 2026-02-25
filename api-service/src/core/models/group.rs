use database_client::{models::Group, schema::group};
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
/// sent to the database for creating a new group
pub struct CreateGroup {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
/// returned from the database when displaying a group
pub struct DisplayGroup {
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
#[diesel(table_name = group)]
/// sent to the database when updating a group
pub struct UpdateGroup {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: Option<String>,
}

impl From<CreateGroup> for Group {
    fn from(value: CreateGroup) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
        }
    }
}
