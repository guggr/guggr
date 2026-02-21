use database_client::{models::User, schema::user};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric)]
pub struct DisplayUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, AsChangeset, LabelledGeneric,
)]
#[diesel(table_name = user)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
            email: value.email,
            password: value.password,
        }
    }
}

#[cfg(test)]
mod tests {
    use frunk::labelled::Transmogrifier;

    use super::*;

    #[test]
    fn test_transmorg() {
        let u = User {
            id: "abc".to_string(),
            name: "john".to_string(),
            email: "bogus".to_string(),
            password: "secret".to_string(),
        };
        let d: DisplayUser = u.transmogrify();
        assert_eq!(
            d,
            DisplayUser {
                id: "abc".to_string(),
                name: "john".to_string(),
                email: "bogus".to_string(),
            }
        )
    }
}
