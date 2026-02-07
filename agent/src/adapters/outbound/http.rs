use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, anyhow};
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::HttpJobResult, v1::JobResult},
};
use protocheck::types::{Duration, Timestamp};
use tracing::info;

use crate::core::{ports::monitor::MonitorPort, service::jobservice::JobServiceError};

pub struct HttpAdapter {
    client: reqwest::Client,
}

impl HttpAdapter {
    pub fn new() -> Self {
        HttpAdapter {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl MonitorPort for HttpAdapter {
    async fn execute(&self, job: &Job) -> anyhow::Result<JobResult, JobServiceError> {
        let http_details = job.http.as_ref().unwrap();

        info!(
            "executing http job with id {} for host {}",
            job.id, http_details.url
        );

        let start_time = Instant::now();
        let response = self.client.head(&http_details.url).send().await;
        let latency = start_time.elapsed();
        let protocheck_latency = Duration {
            seconds: latency.as_secs() as i64,
            nanos: latency.as_nanos() as i32,
        };

        let (res, reachable) = match response {
            Ok(res) => (Some(res), true),
            Err(error) => {
                if self.client.head("http://gug.gr").send().await.is_err() {
                    // Error on agent side return agent error
                    return Err(JobServiceError::AgentIssue(anyhow!(error)));
                } else {
                    (None, false)
                }
            }
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?;
        let job_result = match res {
            Some(response) => {
                let remote_ip = response
                    .remote_addr()
                    .context("receiving remote address")
                    .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?
                    .ip()
                    .to_string()
                    .as_bytes()
                    .to_vec();
                let status_code = response.status().as_u16() as i32;
                let payload = response
                    .bytes()
                    .await
                    .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?;

                JobResult {
                    id: job.id.clone(),
                    timestamp: Some(Timestamp {
                        seconds: timestamp.as_secs() as i64,
                        ..Default::default()
                    }),
                    http: Some(HttpJobResult {
                        reachable,
                        ip_address: remote_ip,
                        status_code,
                        latency: Some(protocheck_latency),
                        payload: payload.to_vec(),
                    }),
                    ..Default::default()
                }
            }
            None => JobResult {
                id: job.id.clone(),
                timestamp: Some(Timestamp {
                    seconds: timestamp.as_secs() as i64,
                    ..Default::default()
                }),
                http: Some(HttpJobResult {
                    reachable,
                    ip_address: vec![],
                    status_code: 0,
                    latency: None,
                    payload: vec![],
                }),
                ..Default::default()
            },
        };

        Ok(job_result)
    }
}
