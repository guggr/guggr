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
/// Returned user
pub struct DisplayGroupMember {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
/// Returned Group
pub struct DisplayGroup {
    pub id: String,
    pub name: String,
    pub members: Vec<DisplayGroupMember>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, LabelledGeneric, ToSchema, Default)]
/// Struct to Update a Group
pub struct UpdateRequestGroup {
    pub name: String,
    pub members: Vec<DisplayGroupMember>,
}

impl From<CreateGroup> for Group {
    fn from(value: CreateGroup) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
        }
    }
}

impl DisplayGroup {
    pub fn from_group(value: Group, members: Vec<DisplayGroupMember>) -> Self {
        Self {
            id: value.id,
            name: value.name,
            members,
        }
    }
}
