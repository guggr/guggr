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
    ///
    /// # Errors
    ///
    /// Raises a [`TypeMapperError`] if the IP Address / Latency could not be
    /// converted
    fn from_protobuf_type(run_id: &str, value: &HttpJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(&value.ip_address)
                .map_err(TypeMapperError::IpAddress)?,
            status_code: value.status_code,
            latency: protocheck_duration_to_i32_millis(
                value
                    .latency
                    .ok_or_else(|| TypeMapperError::Latency("latency is missing".to_string()))?,
            )
            .map_err(|err| TypeMapperError::Latency(err.to_string()))?,
            payload: value.payload.clone(),
        })
    }
}

impl FromProtobufTypeJobResult<&PingJobResult> for db_models::JobResultPing {
    /// Maps protobuf [`PingJobResult`] to its respective database model
    ///
    /// # Errors
    ///
    /// Raises a [`TypeMapperError`] if the IP Address / Latency could not be
    /// converted
    fn from_protobuf_type(run_id: &str, value: &PingJobResult) -> Result<Self, TypeMapperError> {
        Ok(Self {
            id: run_id.to_string(),
            ip_address: ipnet_from_bytes_host(value.ip_address.as_slice())
                .map_err(TypeMapperError::IpAddress)?,
            latency: protocheck_duration_to_i32_millis(
                value
                    .latency
                    .ok_or_else(|| TypeMapperError::Latency("latency is missing".to_string()))?,
            )
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
    ///
    /// # Errors
    ///
    /// Raises a [`TypeMapperError`] if the IP Address / Latency could not be
    /// converted
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
            timestamp: naive_from_proto_ts(
                &value.timestamp.ok_or_else(|| {
                    TypeMapperError::Timestamp("timestamp is missing".to_string())
                })?,
            )
            .map_err(|e| TypeMapperError::Timestamp(e))?,
            reachable,
        })
    }
}

#[cfg(test)]
mod tests {

    use std::{net::Ipv4Addr, vec};

    use database_client::models::{JobResultHttp, JobResultPing, JobRun};
    use gen_proto_types::job_result::types::v1::{HttpJobResult, PingJobResult};
    use ipnet::Ipv4Net;
    use protocheck::types::Timestamp;

    use super::*;
    use crate::core::domain::type_mapper::{FromProtobufType, FromProtobufTypeJobResult};

    fn mock_ping_result(
        reachable: bool,
        ip: Vec<u8>,
        latency: protocheck::types::Duration,
    ) -> PingJobResult {
        PingJobResult {
            reachable,
            ip_address: ip,
            latency: Some(latency),
        }
    }

    fn mock_http_result(
        reachable: bool,
        ip: Vec<u8>,
        latency: protocheck::types::Duration,
        status_code: i32,
    ) -> HttpJobResult {
        HttpJobResult {
            reachable,
            ip_address: ip,
            status_code,
            latency: Some(latency),
            payload: vec![],
        }
    }

    fn mock_result(id: String, timestamp: Timestamp) -> JobResult {
        JobResult {
            id,
            timestamp: Some(timestamp),
            batch_id: "abcd".to_string(),
            run_id: "abcd".to_string(),
            http: None,
            ping: None,
        }
    }

    #[test]
    fn invalid_ip() {
        let ping_job =
            mock_ping_result(false, vec![1, 1, 1], protocheck::types::Duration::new(5, 5));

        let err = JobResultPing::from_protobuf_type("abc", &ping_job).unwrap_err();
        assert_eq!(
            err,
            TypeMapperError::IpAddress("ip bytes must be 4 (v4) or 16 (v6), got: 3".to_string())
        );
    }

    #[test]
    fn invalid_latency() {
        let ping_job = mock_ping_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(i64::MAX, 0),
        );

        let err = JobResultPing::from_protobuf_type("abc", &ping_job).unwrap_err();
        assert_eq!(
            err,
            TypeMapperError::Latency("duration too large for i32 milliseconds".to_string())
        );
    }

    #[test]
    fn invalid_timestamp() {
        let ping_job = mock_ping_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(0, 0),
        );
        let mut job = mock_result(
            "disabled".to_string(),
            Timestamp {
                seconds: i64::MAX,
                nanos: 0,
            },
        );
        job.ping = Some(ping_job);

        let err = JobRun::from_protobuf_type(false, false, &job).unwrap_err();
        assert_eq!(
            err,
            TypeMapperError::Timestamp(
                "Could not create a NaiveDateTime from +292277026596-12-04T15:30:07Z".to_string()
            )
        )
    }

    #[test]
    fn valid_ping_job() {
        let ping_job = mock_ping_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(5, 5),
        );
        let expected = JobResultPing {
            id: "abc".to_string(),
            ip_address: ipnet::IpNet::V4(Ipv4Net::new(Ipv4Addr::new(1, 1, 1, 1), 32).unwrap()),
            latency: 5000,
        };

        let res = JobResultPing::from_protobuf_type("abc", &ping_job).unwrap();
        assert_eq!(res.id, expected.id);
        assert_eq!(res.ip_address, expected.ip_address);
        assert_eq!(res.latency, expected.latency);
    }

    #[test]
    fn valid_http_job() {
        let http_job = mock_http_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(5, 5),
            200,
        );
        let expected = JobResultHttp {
            id: "abc".to_string(),
            ip_address: ipnet::IpNet::V4(Ipv4Net::new(Ipv4Addr::new(1, 1, 1, 1), 32).unwrap()),
            latency: 5000,
            status_code: 200,
            payload: vec![],
        };

        let res = JobResultHttp::from_protobuf_type("abc", &http_job).unwrap();
        assert_eq!(res.id, expected.id);
        assert_eq!(res.ip_address, expected.ip_address);
        assert_eq!(res.latency, expected.latency);
        assert_eq!(res.status_code, expected.status_code);
        assert_eq!(res.payload, expected.payload);
    }
}
