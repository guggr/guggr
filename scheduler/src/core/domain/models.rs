use chrono::{Duration, NaiveDateTime};

pub struct Job {
    pub id: String,
    pub name: String,
    pub job_type_id: String,
    pub group_id: String,
    pub notify_users: bool,
    pub custom_notification: Option<String>,
    pub run_every: Duration,
    pub last_scheduled: Option<NaiveDateTime>,
}
