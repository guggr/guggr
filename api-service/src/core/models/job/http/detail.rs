use database_client::{models::JobDetailsHttp, schema::job_details_http};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Default, Deserialize,
)]
pub struct CreateJobDetailsHttp {
    pub url: String,
}

#[derive(
    Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Default, Serialize,
)]
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

impl From<CreateJobDetailsHttp> for JobDetailsHttp {
    fn from(value: CreateJobDetailsHttp) -> Self {
        Self {
            id: nanoid::nanoid!(),
            url: value.url,
        }
    }
}
