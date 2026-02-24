use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use database_client::{models::User, schema::user};
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
pub struct CreateUser {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: String,
    #[garde(email)]
    #[schema(format = "email")]
    pub email: String,
    #[garde(ascii, length(min = 8))]
    #[schema(min_length = 8, format = "password")]
    pub password: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, Serialize, Deserialize, ToSchema, LabelledGeneric, Default,
)]
pub struct DisplayUser {
    pub id: String,
    pub name: String,
    pub email: String,
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
pub struct UpdateUser {
    #[garde(ascii, length(min = 1))]
    #[schema(min_length = 1)]
    pub name: Option<String>,
    #[garde(email)]
    #[schema(format = "email")]
    pub email: Option<String>,
    #[garde(ascii, length(min = 8))]
    #[schema(min_length = 8, format = "password")]
    pub password: Option<String>,
}

impl TryFrom<CreateUser> for User {
    type Error = argon2::password_hash::Error;
    fn try_from(value: CreateUser) -> Result<Self, Self::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let pwhash = argon2
            .hash_password(value.password.as_bytes(), &salt)?
            .to_string();
        Ok(Self {
            id: nanoid::nanoid!(),
            name: value.name,
            email: value.email,
            password: pwhash,
            jwt_secret: nanoid::nanoid!(32),
            jwt_salt: nanoid::nanoid!(16),
        })
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
            jwt_secret: "secret2".to_string(),
            jwt_salt: "salt".to_string(),
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
