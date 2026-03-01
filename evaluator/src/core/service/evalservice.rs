use std::sync::Arc;

use gen_proto_types::job_result::v1::JobResult;

use crate::core::{domain::errors::JobEvaluatorError, ports::storage::StoragePort};

pub struct EvalService {
    postgres_adapter: Arc<dyn StoragePort>,
}

impl Clone for EvalService {
    fn clone(&self) -> Self {
        Self {
            postgres_adapter: Arc::clone(&self.postgres_adapter),
        }
    }
}

impl EvalService {
    pub fn new(postgres_adapter: Arc<dyn StoragePort>) -> Self {
        Self { postgres_adapter }
    }
    /// Evaluates a protobuf `JobResult`, conditionally notifies and dumps the
    /// Result into the database
    ///
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
    use gen_proto_types::{
        job_result::types::v1::{HttpJobResult, PingJobResult},
        job_types::v1::JobType,
    };
    use ipnet::Ipv4Net;
    use protify::proto_types::Timestamp;

    use super::*;
    use crate::{
        adapters::outbound::postgres::PostgresAdapterError,
        core::domain::type_mapper::{FromProtobufType, FromProtobufTypeJobResult},
        logging::init_tracing,
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
    impl StoragePort for MockDatabase {
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
            let reachable = match &job_result.job_type() {
                JobType::Http => {
                    if let Some(http) = &job_result.http {
                        self.write_job_result_http(&job_result.run_id, http)?;
                    } else {
                        return Err(JobEvaluatorError::Internal(format!(
                            "expected job with job id '{}', run id '{}' and HTTP type to contain a HTTP job result body, but it did not",
                            job_result.id, job_result.run_id
                        )));
                    }
                    job_result.http.as_ref().is_some_and(|r| r.reachable)
                }
                JobType::Ping => {
                    if let Some(ping) = &job_result.ping {
                        self.write_job_result_ping(&job_result.run_id, ping)?;
                    } else {
                        return Err(JobEvaluatorError::Internal(format!(
                            "expected job with job id '{}', run id '{}' and ping type to contain a ping job result body, but it did not",
                            job_result.id, job_result.run_id
                        )));
                    }
                    job_result.ping.as_ref().is_some_and(|r| r.reachable)
                }
                JobType::Unspecified => {
                    return Err(JobEvaluatorError::Internal(format!(
                        "received unspecified job type for run id '{}', job id '{}'",
                        job_result.run_id, job_result.id,
                    )));
                }
            };

            let job_run = JobRun::from_protobuf_type(notified, reachable, job_result)
                .map_err(PostgresAdapterError::from)?;
            *self.job_result.lock().unwrap() = Some(job_run);

            Ok(())
        }
    }

    fn mock_ping_result(
        reachable: bool,
        ip: Vec<u8>,
        latency: protify::proto_types::Duration,
    ) -> PingJobResult {
        PingJobResult {
            reachable,
            ip_address: ip.into(),
            latency: Some(latency),
        }
    }

    fn mock_http_result(
        reachable: bool,
        ip: Vec<u8>,
        latency: protify::proto_types::Duration,
        status_code: i32,
    ) -> HttpJobResult {
        HttpJobResult {
            reachable,
            ip_address: ip.into(),
            status_code,
            latency: Some(latency),
            payload: vec![].into(),
        }
    }

    fn mock_result(id: String, job_type: JobType, timestamp: Timestamp) -> JobResult {
        JobResult {
            id,
            timestamp: Some(timestamp),
            batch_id: "abcd".to_string(),
            run_id: "abcd".to_string(),
            job_type: job_type.into(),
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
            JobType::Unspecified,
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );
        let err = service.evaluate_job_result(&job).await.unwrap_err();
        assert_eq!(err, JobEvaluatorError::Internal("bogus".to_string()))
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
            protify::proto_types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "enabled".to_string(),
            JobType::Ping,
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
            protify::proto_types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "disabled".to_string(),
            JobType::Ping,
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
    async fn empty_job() {
        init();

        let mock_adapter = Arc::from(MockDatabase::new());

        let service = EvalService::new(mock_adapter);
        let job = mock_result(
            "enabled".to_string(),
            JobType::Http,
            Timestamp {
                seconds: 0,
                nanos: 0,
            },
        );

        let err = service.evaluate_job_result(&job).await.unwrap_err();
        assert_eq!(err, JobEvaluatorError::Internal("expected job with job id 'enabled', run id 'abcd' and HTTP type to contain a HTTP job result body, but it did not".to_string()));
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
            protify::proto_types::Duration::new(5, 5),
        );
        let mut job = mock_result(
            "disabled".to_string(),
            JobType::Ping,
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
        assert_eq!(actual, &expected);
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
            protify::proto_types::Duration::new(5, 5),
            200,
        );
        let mut job = mock_result(
            "disabled".to_string(),
            JobType::Http,
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
        assert_eq!(actual, &expected);
    }
}
