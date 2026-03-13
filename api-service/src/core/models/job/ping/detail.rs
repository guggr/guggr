use database_client::{models::JobDetailsPing, schema::job_details_ping};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Default, Deserialize,
)]
/// Struct to create a new Job Ping Detail
pub struct CreateJobDetailsPing {
    pub host: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Default, Serialize,
)]
/// Returned Job Ping Detail
pub struct DisplayJobDetailsPing {
    pub id: String,
    pub host: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, AsChangeset, Deserialize, ToSchema, Default,
)]
#[diesel(table_name = job_details_ping)]
/// Struct to update a new Job Ping Detail
pub struct UpdateJobDetailsPing {
    pub host: Option<String>,
}

impl CreateJobDetailsPing {
    pub fn from_create_detail(id: &str, detail: CreateJobDetailsPing) -> JobDetailsPing {
        JobDetailsPing {
            id: id.to_string(),
            host: detail.host,
        }
    }
}
