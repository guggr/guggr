use std::{
    net::IpAddr,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use agent::ToProto;
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::HttpJobResult, v1::JobResult},
};
use tracing::info;

use crate::core::{
    ports::monitor::MonitorPort,
    service::jobservice::{AgentError, JobServiceError},
};

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
    async fn execute(&self, job: &Job, run_id: String) -> Result<JobResult, JobServiceError> {
        let http_details = job.http.as_ref().unwrap();

        info!(
            "executing http job with id {} for host {}",
            job.id, http_details.url
        );

        let start_time = Instant::now();
        let response = self.client.head(&http_details.url).send().await;
        let latency = start_time.elapsed();

        let (res, reachable) = match response {
            Ok(res) => (Some(res), true),
            Err(error) => {
                if self.client.head("http://gug.gr").send().await.is_err() {
                    // Error on agent side return agent error
                    return Err(JobServiceError::AgentIssue(AgentError::Http(error).into()));
                } else {
                    (None, false)
                }
            }
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| JobServiceError::AgentIssue(e.into()))?;

        let http_job_result = match res {
            Some(response) => {
                let remote_ip = response
                    .remote_addr()
                    .ok_or_else(|| JobServiceError::AgentIssue(AgentError::RemoteAddress.into()))?
                    .ip();
                let ip_bytes = match remote_ip {
                    IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
                    IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
                };
                let status_code = response.status().as_u16() as i32;
                let payload = response
                    .bytes()
                    .await
                    .map_err(|e| JobServiceError::AgentIssue(AgentError::Http(e).into()))?;

                HttpJobResult {
                    reachable,
                    ip_address: ip_bytes,
                    status_code,
                    latency: Some(latency.to_proto()),
                    payload: payload.to_vec(),
                }
            }
            None => HttpJobResult {
                reachable,
                ..Default::default()
            },
        };

        let job_result = JobResult {
            id: job.id.clone(),
            batch_id: job.batch_id.clone(),
            run_id,
            timestamp: Some(timestamp.to_proto()),
            http: Some(http_job_result),
            ..Default::default()
        };

        Ok(job_result)
    }
}

#[cfg(test)]
mod tests {
    use gen_proto_types::job::{types::v1::HttpJobType, v1::JobType};
    use httpmock::{Method::HEAD, MockServer};

    use super::*;

    #[tokio::test]
    async fn test_http_success() {
        let server = MockServer::start();

        let mock = server.mock(|when, then| {
            when.method(HEAD).path("/");
            then.status(200);
        });

        let job = Job {
            id: "GyLQDBZm1JYP7f_eJ24iH".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Http.into(),
            http: Some(HttpJobType {
                url: server.url("/"),
            }),
            ..Default::default()
        };

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let http_adapter = HttpAdapter::new();
        let res: JobResult = http_adapter.execute(&job, run_id.clone()).await.unwrap();
        mock.assert();
        assert_eq!(
            res,
            JobResult {
                id: "GyLQDBZm1JYP7f_eJ24iH".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                http: Some(HttpJobResult {
                    reachable: true,
                    ip_address: vec![127, 0, 0, 1],
                    status_code: 200,
                    latency: res.http.as_ref().unwrap().latency,
                    payload: vec![],
                }),
                ..Default::default()
            }
        )
    }

    #[tokio::test]
    async fn test_http_failure() {
        let job = Job {
            id: "S3tqA6Gb-eY-jMIcGo7Is".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Http.into(),
            http: Some(HttpJobType {
                url: "http://example.lol".to_string(),
            }),
            ..Default::default()
        };

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let http_adapter = HttpAdapter::new();
        let res = http_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "S3tqA6Gb-eY-jMIcGo7Is".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                http: Some(HttpJobResult {
                    reachable: false,
                    ip_address: vec![],
                    status_code: 0,
                    payload: vec![],
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
    }
}
