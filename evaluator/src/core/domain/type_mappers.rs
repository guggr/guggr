use database_client::models as db_models;
use gen_proto_types::job_result::{
    types::v1::{HttpJobResult, PingJobResult},
    v1::JobResult,
};

use crate::{
    core::domain::errors::TypeMapperError, ipnet_from_bytes_host, naive_from_proto_ts,
    protocheck_duration_to_i32_millis,
};
/// Trait for converting protobuf [`JobResult`] sub-types to database types.
pub trait FromProtobufTypeJobResult<F> {
    /// # Errors
    ///
    /// Raises a [`TypeMapperError`] if the IP Address / Latency could not be
    /// converted
    fn from_protobuf_type(run_id: &str, value: F) -> Result<Self, TypeMapperError>
    where
        Self: Sized;
}

impl FromProtobufTypeJobResult<&HttpJobResult> for db_models::JobResultHttp {
    /// Maps protobuf [`HttpJobResult`] to its respective database model
    fn from_protobuf_type(run_id: &str, value: &HttpJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(&value.ip_address)
                .map_err(|err| TypeMapperError::IpAddress(err.to_string()))?,
            status_code: value.status_code,
            latency: protocheck_duration_to_i32_millis(value.latency.unwrap())
                .map_err(|err| TypeMapperError::Latency(err.to_string()))?,
            payload: value.payload.clone(),
        })
    }
}

impl FromProtobufTypeJobResult<&PingJobResult> for db_models::JobResultPing {
    /// Maps protobuf [`PingJobResult`] to its respective database model
    fn from_protobuf_type(run_id: &str, value: &PingJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(value.ip_address.as_slice())
                .map_err(|err| TypeMapperError::IpAddress(err.to_string()))?,
            latency: protocheck_duration_to_i32_millis(value.latency.unwrap())
                .map_err(|err| TypeMapperError::Latency(err.to_string()))?,
        })
    }
}

/// Trait for converting the protobuf [`JobResult`] type to its database type.
pub trait FromProtobufType<F> {
    /// # Errors
    ///
    /// Raises a [`TypeMapperError`] if the timestamp could not be converted
    fn from_protobuf_type(
        notified: bool,
        reachable: bool,
        value: F,
    ) -> Result<Self, TypeMapperError>
    where
        Self: Sized;
}

impl FromProtobufType<&JobResult> for db_models::JobRun {
    /// Maps protobuf [`JobResult`] to its respective database model
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
