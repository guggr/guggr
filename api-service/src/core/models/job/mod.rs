use chrono::{Duration, NaiveDateTime};
use database_client::{models::Job, schema::job};
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

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema)]
pub struct CreateJob {
    pub name: String,
    pub job_type_id: String,
    pub group_id: String,
    pub notify_users: bool,
    pub custom_notification: Option<String>,
    pub run_every: Duration,
    pub last_scheduled: Option<NaiveDateTime>,
    pub details: CreateJobDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema, Deserialize)]
pub enum CreateJobDetails {
    Http(CreateJobDetailsHttp),
    Ping(CreateJobDetailsPing),
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Serialize, ToSchema)]
pub struct DisplayJob {
    pub id: String,
    pub name: String,
    pub job_type_id: String,
    pub group_id: String,
    pub notify_users: bool,
    pub custom_notification: Option<String>,
    pub run_every: Duration,
    pub last_scheduled: Option<NaiveDateTime>,
    pub details: DisplayJobDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema, Serialize)]
pub enum DisplayJobDetails {
    Http(DisplayJobDetailsHttp),
    Ping(DisplayJobDetailsPing),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, LabelledGeneric, ToSchema)]
pub struct UpdateJob {
    pub id: Option<String>,
    pub name: Option<String>,
    pub job_type_id: Option<String>,
    pub group_id: Option<String>,
    pub notify_users: Option<bool>,
    pub custom_notification: Option<String>,
    pub run_every: Option<Duration>,
    pub last_scheduled: Option<NaiveDateTime>,
    pub details: Option<UpdateJobDetails>,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, ToSchema)]
pub enum UpdateJobDetails {
    Http(UpdateJobDetailsHttp),
    Ping(UpdateJobDetailsPing),
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, LabelledGeneric, ToSchema, AsChangeset)]
#[diesel(table_name = job)]
pub struct UpdatableJob {
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
            last_scheduled: value.last_scheduled,
        }
    }
}

impl From<Job> for DisplayJob {
    fn from(value: Job) -> Self {
        Self {
            id: value.id,
            name: value.name,
            job_type_id: value.job_type_id,
            group_id: value.group_id,
            notify_users: value.notify_users,
            custom_notification: value.custom_notification,
            run_every: value.run_every,
            last_scheduled: value.last_scheduled,
            details: DisplayJobDetails::Http(DisplayJobDetailsHttp::default()),
        }
    }
}

impl From<UpdateJob> for UpdatableJob {
    fn from(value: UpdateJob) -> Self {
        Self {
            id: value.id,
            name: value.name,
            job_type_id: value.job_type_id,
            group_id: value.group_id,
            notify_users: value.notify_users,
            custom_notification: value.custom_notification,
            run_every: value.run_every,
            last_scheduled: value.last_scheduled,
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
