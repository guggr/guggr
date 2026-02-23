use chrono::NaiveDateTime;
use database_client::models::JobRun;
use frunk::LabelledGeneric;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::core::models::job::{
    http::result::DisplayJobResultHttp, ping::result::DisplayJobResultPing,
};

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, Serialize, ToSchema)]
pub struct DisplayJobRun {
    pub id: String,
    pub job_id: String,
    pub timestamp: NaiveDateTime,
    pub triggered_notification: bool,
    pub batch_id: String,
    pub reachable: bool,
    pub details: DisplayJobRunDetails,
}

#[derive(Debug, PartialEq, Eq, Clone, LabelledGeneric, Deserialize, Serialize, ToSchema)]
pub enum DisplayJobRunDetails {
    Http(DisplayJobResultHttp),
    Ping(DisplayJobResultPing),
}

impl From<JobRun> for DisplayJobRun {
    fn from(value: JobRun) -> Self {
        Self {
            id: value.id,
            job_id: value.job_id,
            timestamp: value.timestamp,
            triggered_notification: value.triggered_notification,
            batch_id: value.batch_id,
            reachable: value.reachable,
            details: DisplayJobRunDetails::Http(DisplayJobResultHttp::default()),
        }
    }
}
