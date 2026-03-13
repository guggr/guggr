use chrono::{DateTime, NaiveDateTime, Utc};
use database_client::models::JobRun;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::models::job::{
    http::result::DisplayJobResultHttp, ping::result::DisplayJobResultPing,
};

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, Serialize, ToSchema)]
/// Returned JobRun
pub struct DisplayJobRun {
    pub id: String,
    pub job_id: String,
    pub timestamp: DateTime<Utc>,
    pub triggered_notification: bool,
    pub batch_id: String,
    pub reachable: bool,
    pub details: DisplayJobRunDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, Serialize, ToSchema)]
/// Returned JobRun details
pub enum DisplayJobRunDetails {
    #[serde(rename = "http")]
    Http(DisplayJobResultHttp),
    #[serde(rename = "ping")]
    Ping(DisplayJobResultPing),
    #[serde(rename = "undefined")]
    Undefined,
}

impl From<JobRun> for DisplayJobRun {
    fn from(value: JobRun) -> Self {
        Self {
            id: value.id,
            job_id: value.job_id,
            timestamp: value.timestamp.and_utc(),
            triggered_notification: value.triggered_notification,
            batch_id: value.batch_id,
            reachable: value.reachable,
            details: DisplayJobRunDetails::Undefined,
        }
    }
}
