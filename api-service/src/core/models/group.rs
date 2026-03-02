use database_client::models::Group;
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
/// Struct to create a new Group
pub struct CreateGroup {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
/// Returned Group
pub struct DisplayGroup {
    pub id: String,
    pub name: String,
}

impl From<CreateGroup> for Group {
    fn from(value: CreateGroup) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
        }
    }
}
