use database_client::models as db_models;
use gen_proto_types::job_result::{
    types::v1::{HttpJobResult, PingJobResult},
    v1::JobResult,
};

use crate::{
    core::domain::errors::TypeMapperError, ipnet_from_bytes_host, naive_from_proto_ts,
    protocheck_duration_to_i32_millis,
};
/// Trait for converting protobuf types to database types.
pub trait FromProtobufTypeJobResult<F> {
    fn from_protobuf_type(run_id: &str, value: F) -> Result<Self, TypeMapperError>
    where
        Self: Sized;
}

impl FromProtobufTypeJobResult<&HttpJobResult> for db_models::JobResultHttp {
    /// Maps protobuf `HttpJobResult` to its respective database model
    fn from_protobuf_type(run_id: &str, value: &HttpJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(&value.ip_address)
                .map_err(|err| TypeMapperError::IpAddress(err.to_string()))?,
            status_code: value.status_code,
            latency: protocheck_duration_to_i32_millis(value.latency.unwrap())
                .map_err(|err| TypeMapperError::Timestamp(err.to_string()))?,
            payload: value.payload.clone(),
        })
    }
}

impl FromProtobufTypeJobResult<&PingJobResult> for db_models::JobResultPing {
    //// Maps protobuf `PingJobResult` to its respective database model
    fn from_protobuf_type(run_id: &str, value: &PingJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(value.ip_address.as_slice())
                .map_err(|err| TypeMapperError::IpAddress(err.to_string()))?,
            latency: protocheck_duration_to_i32_millis(value.latency.unwrap())
                .map_err(|err| TypeMapperError::Timestamp(err.to_string()))?,
        })
    }
}

pub trait FromProtobufType<F> {
    fn from_protobuf_type(
        notified: bool,
        reachable: bool,
        value: F,
    ) -> Result<Self, TypeMapperError>
    where
        Self: Sized;
}

impl FromProtobufType<&JobResult> for db_models::JobRun {
    //// Maps protobuf `PingJobResult` to its respective database model
    fn from_protobuf_type(
        notified: bool,
        reachable: bool,
        value: &JobResult,
    ) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: value.run_id.clone(),
            job_id: value.id.clone(),
            batch_id: value.batch_id.clone(),
            triggered_notification: notified,
            // there is always a timestamp so it should be safe to unwrap
            timestamp: naive_from_proto_ts(&value.timestamp.unwrap())
                .ok_or_else(|| TypeMapperError::Timestamp(value.timestamp.unwrap().to_string()))?,
            reachable,
        })
    }
}
