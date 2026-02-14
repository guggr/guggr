use database_client::models as db_models;
use gen_proto_types::job::{
    types::v1::{HttpJobType, PingJobType},
    v1::{Job, JobType},
};

/// Trait for converting Database types to protobuf types.
pub trait FromDatabaseType<F> {
    fn from_database_type(value: F) -> Self;
}

impl FromDatabaseType<String> for JobType {
    /// Maps job types stored in the database to string
    fn from_database_type(job_type_id: String) -> Self {
        match job_type_id.as_str() {
            "http" => Self::Http,
            "ping" => Self::Ping,
            _ => Self::Unspecified,
        }
    }
}

impl FromDatabaseType<db_models::JobDetailsHttp> for HttpJobType {
    /// Maps job details stored in the database to the protobuf model.
    fn from_database_type(value: db_models::JobDetailsHttp) -> Self {
        Self { url: value.url }
    }
}

impl FromDatabaseType<db_models::JobDetailsPing> for PingJobType {
    /// Maps job details stored in the database to the protobuf model.
    fn from_database_type(value: db_models::JobDetailsPing) -> Self {
        Self { host: value.host }
    }
}

pub type DatabaseJobResult = (
    db_models::Job,
    Option<db_models::JobDetailsHttp>,
    Option<db_models::JobDetailsPing>,
);

pub trait JobFromDatabaseJobResult {
    /// Maps job results stored in the database to the protobuf model.
    fn from_database_type(value: DatabaseJobResult, batch_id: String) -> Self;
}

impl JobFromDatabaseJobResult for Job {
    fn from_database_type(value: DatabaseJobResult, batch_id: String) -> Self {
        Self {
            id: value.0.id,
            batch_id: batch_id,
            job_type: JobType::from_database_type(value.0.job_type_id).into(),
            http: value.1.map(FromDatabaseType::from_database_type),
            ping: value.2.map(FromDatabaseType::from_database_type),
        }
    }
}
