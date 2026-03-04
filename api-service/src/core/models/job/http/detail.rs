use database_client::{models::JobDetailsHttp, schema::job_details_http};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Default, Deserialize,
)]
/// Struct to create a new Job HTTP Detail
pub struct CreateJobDetailsHttp {
    pub url: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Default, Serialize,
)]
/// Returned Job HTTP Detail
pub struct DisplayJobDetailsHttp {
    pub id: String,
    pub url: String,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    Clone,
    LabelledGeneric,
    AsChangeset,
    Deserialize,
    ToSchema,
    Default,
    Serialize,
)]
#[diesel(table_name = job_details_http)]
/// Struct to update a new Job HTTP Detail
pub struct UpdateJobDetailsHttp {
    pub url: Option<String>,
}

// can't impl `From` as I need the id of the parent job
pub fn to_job_detail_http(id: &str, j: CreateJobDetailsHttp) -> JobDetailsHttp {
    JobDetailsHttp {
        id: id.to_string(),
        url: j.url,
    }
}
