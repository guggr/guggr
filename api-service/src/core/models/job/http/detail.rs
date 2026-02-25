use database_client::schema::job_details_http;
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Default, Deserialize,
)]
/// sent to the database for creating a new job detail
pub struct CreateJobDetailsHttp {
    pub url: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Default, Serialize,
)]
/// returned from the database when displaying a job detail
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
pub struct UpdateJobDetailsHttp {
    pub url: Option<String>,
}

// can't impl `From` as I need the id of the parent job
pub fn to_job_detail_http(id: &str, j: CreateJobDetailsHttp) -> DisplayJobDetailsHttp {
    DisplayJobDetailsHttp {
        id: id.to_string(),
        url: j.url,
    }
}
