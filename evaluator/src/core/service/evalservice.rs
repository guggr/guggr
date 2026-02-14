use std::sync::Arc;

use gen_proto_types::job_result::v1::JobResult;

use crate::core::{domain::errors::JobEvaluatorError, ports::database::DatabasePort};

pub struct EvalService {
    postgres_adapter: Arc<dyn DatabasePort + Send + Sync>,
}

impl Clone for EvalService {
    fn clone(&self) -> Self {
        Self {
            postgres_adapter: Arc::clone(&self.postgres_adapter),
        }
    }
}

impl EvalService {
    pub fn new(postgres_adapter: Arc<dyn DatabasePort + Send + Sync>) -> Self {
        Self { postgres_adapter }
    }

    /// # Errors
    ///
    /// Will return `Err` if the `notification` setting could not be
    /// retrieved or an issue occurred while writing the result into the
    /// database
    pub async fn evaluate_job_result(
        &self,
        job_result: &JobResult,
    ) -> anyhow::Result<(), JobEvaluatorError> {
        let notify = self
            .postgres_adapter
            .notification_enabled(&job_result.id)
            .await?;
        if notify {
            // dispatch notifier
            // maybe set notify to false on error?
        }

        self.postgres_adapter
            .write_job_result(job_result, notify)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{net::Ipv4Addr, sync::Mutex, vec};

    use async_trait::async_trait;
    use database_client::models::{JobResultHttp, JobResultPing, JobRun};
    use gen_proto_types::job_result::types::v1::{HttpJobResult, PingJobResult};
    use ipnet::Ipv4Net;
    use protocheck::types::Timestamp;

    use super::*;
    use crate::{
        adapters::outbound::postgres::PostgresAdapterError,
        core::domain::type_mapper::{FromProtobufType, FromProtobufTypeJobResult},
        telemetry::init_tracing,
    };

    struct MockDatabase {
        pub notified: Mutex<Option<bool>>,
        pub job_result: Mutex<Option<JobRun>>,
        pub ping_result: Mutex<Option<JobResultPing>>,
        pub http_result: Mutex<Option<JobResultHttp>>,
    }

    impl MockDatabase {
        fn new() -> Self {
            Self {
                notified: Mutex::new(None),
                job_result: Mutex::new(None),
                ping_result: Mutex::new(None),
                http_result: Mutex::new(None),
            }
        }

        fn write_job_result_http(
            &self,
            run_id: &str,
            result: &HttpJobResult,
        ) -> Result<(), PostgresAdapterError> {
            let result = JobResultHttp::from_protobuf_type(run_id, result)?;
            *self.http_result.lock().unwrap() = Some(result);
            Ok(())
        }

        fn write_job_result_ping(
            &self,
            run_id: &str,
            result: &PingJobResult,
        ) -> Result<(), PostgresAdapterError> {
            let result = JobResultPing::from_protobuf_type(run_id, result)?;
            *self.ping_result.lock().unwrap() = Some(result);
            Ok(())
        }
    }

    #[async_trait]
    impl DatabasePort for MockDatabase {
        async fn notification_enabled(&self, job_id: &str) -> Result<bool, JobEvaluatorError> {
            let val = match job_id {
                "enabled" => true,
                "disabled" => false,
                other => return Err(JobEvaluatorError::Internal(other.to_string())),
            };

            *self.notified.lock().unwrap() = Some(val);
            Ok(val)
        }
        async fn write_job_result(
            &self,
            job_result: &JobResult,
            notified: bool,
        ) -> Result<(), JobEvaluatorError> {
            let reachable = if let Some(http) = &job_result.http {
                http.reachable
            } else if let Some(ping) = &job_result.ping {
                ping.reachable
            } else {
                return Err(JobEvaluatorError::Internal(job_result.run_id.clone()));
            };
            let job_run = JobRun::from_protobuf_type(notified, reachable, job_result)
                .map_err(PostgresAdapterError::from)?;
            *self.job_result.lock().unwrap() = Some(job_run);
            if let Some(http) = &job_result.http {
                self.write_job_result_http(&job_result.run_id, http)?;
            } else if let Some(ping) = &job_result.ping {
                self.write_job_result_ping(&job_result.run_id, ping)?;
            }
            Ok(())
        }
    }

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

    fn init() {
        init_tracing();
    }

    #[tokio::test]
    async fn unknown_job_id() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());
        let service = EvalService::new(mock_adapter);
        let job = mock_result(
            "bogus".to_string(),
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        let err = service.evaluate_job_result(&job).await.unwrap_err();
        assert_eq!(
            err.to_string(),
            JobEvaluatorError::Internal("bogus".to_string()).to_string()
        )
    }

    #[tokio::test]
    async fn notification_enabled() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());
        let mock_for_assert = mock_adapter.clone();
        let service = EvalService::new(mock_adapter);
        let ping_job = mock_ping_result(
            false,
            vec![0, 0, 0, 0],
            protocheck::types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "enabled".to_string(),
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        job.ping = Some(ping_job);
        assert!(service.evaluate_job_result(&job).await.is_ok());
        assert_eq!(*mock_for_assert.notified.lock().unwrap(), Some(true));
    }

    #[tokio::test]
    async fn notification_disabled() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());
        let mock_for_assert = mock_adapter.clone();

        let service = EvalService::new(mock_adapter);
        let ping_job = mock_ping_result(
            false,
            vec![0, 0, 0, 0],
            protocheck::types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "disabled".to_string(),
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        job.ping = Some(ping_job);
        assert!(service.evaluate_job_result(&job).await.is_ok());
        assert_eq!(*mock_for_assert.notified.lock().unwrap(), Some(false));
    }

    #[tokio::test]
    async fn ping_job() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());
        let mock_for_assert = mock_adapter.clone();

        let service = EvalService::new(mock_adapter);
        let ping_job = mock_ping_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "disabled".to_string(),
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        job.ping = Some(ping_job);
        let expected = JobResultPing {
            id: "abcd".to_string(),
            ip_address: ipnet::IpNet::V4(Ipv4Net::new(Ipv4Addr::new(1, 1, 1, 1), 32).unwrap()),
            latency: 5000,
        };
        assert!(service.evaluate_job_result(&job).await.is_ok());
        let guard = mock_for_assert.ping_result.lock().unwrap();
        let actual = guard.as_ref().unwrap();
        assert_eq!(actual.id, expected.id);
        assert_eq!(actual.ip_address, expected.ip_address);
        assert_eq!(actual.latency, expected.latency);
    }

    #[tokio::test]
    async fn http_job() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());
        let mock_for_assert = mock_adapter.clone();

        let service = EvalService::new(mock_adapter);
        let http_job = mock_http_result(
            false,
            vec![1, 1, 1, 1],
            protocheck::types::Duration::new(5, 5),
            200,
        );
        let mut job = mock_result(
            "disabled".to_string(),
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        job.http = Some(http_job);
        let expected = JobResultHttp {
            id: "abcd".to_string(),
            ip_address: ipnet::IpNet::V4(Ipv4Net::new(Ipv4Addr::new(1, 1, 1, 1), 32).unwrap()),
            latency: 5000,
            status_code: 200,
            payload: vec![],
        };
        assert!(service.evaluate_job_result(&job).await.is_ok());
        let guard = mock_for_assert.http_result.lock().unwrap();
        let actual = guard.as_ref().unwrap();
        assert_eq!(actual.id, expected.id);
        assert_eq!(actual.ip_address, expected.ip_address);
        assert_eq!(actual.latency, expected.latency);
        assert_eq!(actual.status_code, expected.status_code);
        assert_eq!(actual.payload, expected.payload);
    }
}
