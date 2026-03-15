use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use database_client::{
    models::{Job, JobDetailsHttp, JobDetailsPing},
    schema::job,
};
use diesel::prelude::AsChangeset;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::models::job::{
    http::detail::{CreateJobDetailsHttp, DisplayJobDetailsHttp, UpdateJobDetailsHttp},
    ping::detail::{CreateJobDetailsPing, DisplayJobDetailsPing, UpdateJobDetailsPing},
};

pub mod http;
pub mod ping;
pub mod run;

/// Type returned by list function
pub type JobWithRawDetails = (
    Job,
    Option<bool>,
    Option<JobDetailsHttp>,
    Option<JobDetailsPing>,
);

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema)]
/// Struct to create a new job
pub struct CreateJob {
    pub name: String,
    pub job_type_id: String,
    pub group_id: String,
    pub notify_users: bool,
    pub custom_notification: Option<String>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[schema(value_type = i64, default = 60)]
    pub run_every: Duration,
    pub details: CreateJobDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Deserialize)]
/// Enum to create Job Details
pub enum CreateJobDetails {
    #[serde(rename = "http")]
    Http(CreateJobDetailsHttp),
    #[serde(rename = "ping")]
    Ping(CreateJobDetailsPing),
}
#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema)]
/// Returned Job
pub struct DisplayJob {
    pub id: String,
    pub name: String,
    pub job_type_id: String,
    pub group_id: String,
    pub notify_users: bool,
    pub custom_notification: Option<String>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    #[schema(value_type = i64, default = 60)]
    pub run_every: Duration,
    pub last_scheduled: Option<DateTime<Utc>>,
    pub reachable: bool,
    pub details: DisplayJobDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Serialize)]
/// Returned Job Details
pub enum DisplayJobDetails {
    #[serde(rename = "http")]
    Http(DisplayJobDetailsHttp),
    #[serde(rename = "ping")]
    Ping(DisplayJobDetailsPing),
    #[serde(rename = "undefined")]
    Undefined,
}
#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, LabelledGeneric, ToSchema)]
/// Struct to Update a Job
pub struct UpdateRequestJob {
    pub id: Option<String>,
    pub name: Option<String>,
    pub job_type_id: Option<String>,
    pub group_id: Option<String>,
    pub notify_users: Option<bool>,
    pub custom_notification: Option<String>,
    #[serde_as(as = "Option<serde_with::DurationSeconds<i64>>")]
    #[schema(value_type = Option<i64>, default = 60)]
    pub run_every: Option<Duration>,
    pub last_scheduled: Option<NaiveDateTime>,
    pub details: Option<UpdateRequestJobDetails>,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema)]
/// Struct to update a job detail
pub enum UpdateRequestJobDetails {
    #[serde(rename = "http")]
    Http(UpdateJobDetailsHttp),
    #[serde(rename = "ping")]
    Ping(UpdateJobDetailsPing),
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, AsChangeset)]
#[diesel(table_name = job)]
/// Struct to Update a Job
pub struct UpdateJob {
    pub id: Option<String>,
    pub name: Option<String>,
    pub job_type_id: Option<String>,
    pub group_id: Option<String>,
    pub notify_users: Option<bool>,
    pub custom_notification: Option<String>,
    pub run_every: Option<Duration>,
    pub last_scheduled: Option<NaiveDateTime>,
}

impl From<CreateJob> for Job {
    fn from(value: CreateJob) -> Self {
        Self {
            id: nanoid::nanoid!(),
            name: value.name,
            job_type_id: value.job_type_id,
            group_id: value.group_id,
            notify_users: value.notify_users,
            custom_notification: value.custom_notification,
            run_every: value.run_every,
            last_scheduled: None,
        }
    }
}

impl DisplayJob {
    pub fn from_job(value: Job, reachable: bool) -> Self {
        Self {
            id: value.id,
            name: value.name,
            job_type_id: value.job_type_id,
            group_id: value.group_id,
            notify_users: value.notify_users,
            custom_notification: value.custom_notification,
            run_every: value.run_every,
            last_scheduled: value.last_scheduled.map(|t| t.and_utc()),
            reachable,
            details: DisplayJobDetails::Undefined,
        }
    }
}

impl From<DisplayJobDetailsHttp> for DisplayJobDetails {
    fn from(value: DisplayJobDetailsHttp) -> Self {
        Self::Http(value)
    }
}

impl From<DisplayJobDetailsPing> for DisplayJobDetails {
    fn from(value: DisplayJobDetailsPing) -> Self {
        Self::Ping(value)
    }
}
