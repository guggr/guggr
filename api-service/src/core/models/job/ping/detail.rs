use database_client::{models::JobDetailsPing, schema::job_details_ping};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Default, Deserialize,
)]
pub struct CreateJobDetailsPing {
    pub host: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Default, Serialize,
)]
pub struct DisplayJobDetailsPing {
    pub id: String,
    pub host: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, AsChangeset, Deserialize, ToSchema, Default,
)]
#[diesel(table_name = job_details_ping)]
pub struct UpdateJobDetailsPing {
    pub host: Option<String>,
}

impl From<CreateJobDetailsPing> for JobDetailsPing {
    fn from(value: CreateJobDetailsPing) -> Self {
        Self {
            id: nanoid::nanoid!(),
            host: value.host,
        }
    }
}
