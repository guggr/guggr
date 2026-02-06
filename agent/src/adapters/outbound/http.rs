use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::HttpJobResult, v1::JobResult},
};
use protocheck::types::{Duration, Timestamp};
use tracing::info;

use crate::core::ports::monitor::MonitorPort;

pub struct HttpAdapter {
    client: reqwest::Client,
}

impl HttpAdapter {
    pub fn new() -> Self {
        return HttpAdapter {
            client: reqwest::Client::new(),
        };
    }
}

#[async_trait]
impl MonitorPort for HttpAdapter {
    async fn execute(&self, job: &Job) -> anyhow::Result<JobResult> {
        let http_details = job.http.as_ref().unwrap();

        info!(
            "executing http job with id {} for host {}",
            job.id, http_details.url
        );

        let start_time = Instant::now();
        // TODO: error handling and checking whether target or agent has issues
        let resp = self.client.head(&http_details.url).send().await?;
        let latency = start_time.elapsed();
        let protocheck_latency = Duration {
            seconds: latency.as_secs() as i64,
            nanos: latency.as_nanos() as i32,
        };

        let remote_ip = resp
            .remote_addr()
            .context("receiving remote address")?
            .ip()
            .to_string()
            .as_bytes()
            .to_vec();
        let status_code = resp.status().as_u16() as i32;
        let payload = resp.bytes().await?;

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let job_result = JobResult {
            id: job.id.clone(),
            timestamp: Some(Timestamp {
                seconds: timestamp.as_secs() as i64,
                ..Default::default()
            }),
            http: Some(HttpJobResult {
                reachable: true,
                ip_address: remote_ip,
                status_code: status_code,
                latency: Some(protocheck_latency),
                payload: payload.to_vec(),
            }),
            ..Default::default()
        };

        Ok(job_result)
    }
}
