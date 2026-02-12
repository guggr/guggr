use std::{
    net::IpAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use agent::ToProto;
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::PingJobResult, v1::JobResult},
};
use protocheck::types::Timestamp;
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tracing::{debug, error, info};

use crate::core::{
    ports::monitor::MonitorPort,
    service::jobservice::{AgentError, JobServiceError},
};

pub struct PingAdapter {}

impl PingAdapter {
    pub fn new() -> Self {
        PingAdapter {}
    }
}

#[async_trait]
impl MonitorPort for PingAdapter {
    async fn execute(&self, job: &Job, run_id: String) -> Result<JobResult, JobServiceError> {
        let ping_details = job.ping.as_ref().unwrap();

        info!(
            "executing ping job with id {} for host {}",
            job.id, ping_details.host
        );

        let client = Client::new(&Config::default())
            .map_err(|e| JobServiceError::AgentIssue(AgentError::Ping(e.into()).into()))?;

        // Check if domain provided or IP
        let host_ip = if let Ok(ip) = ping_details.host.parse::<IpAddr>() {
            ip
        } else {
            debug!("ping adapter received domain. trying to resolve it...");
            if let Some(ip) = agent::resolve_domain(ping_details.host.clone()).await {
                ip
            } else {
                error!(
                    "ping adapter could not resolve domain {}",
                    ping_details.host
                );
                return Err(JobServiceError::AgentIssue(
                    "could not resolve domain for ping job".into(),
                ));
            }
        };

        let mut pinger = client.pinger(host_ip, PingIdentifier(random())).await;

        pinger.timeout(Duration::from_secs(1));

        let job_result = match pinger.ping(PingSequence(0), &[0; 8]).await {
            Ok((packet, latency)) => {
                info!(
                    "received ping {} from {}",
                    match packet {
                        IcmpPacket::V4(_) => "v4",
                        IcmpPacket::V6(_) => "v6",
                    },
                    ping_details.host
                );
                JobResult {
                    id: job.id.clone(),
                    batch_id: job.batch_id.clone(),
                    run_id,
                    timestamp: Some(get_timestamp()?),
                    ping: Some(PingJobResult {
                        reachable: true,
                        ip_address: match packet {
                            IcmpPacket::V4(packet) => packet.get_real_dest().octets().to_vec(),
                            IcmpPacket::V6(packet) => packet.get_real_dest().octets().to_vec(),
                        },
                        latency: Some(latency.to_proto()),
                    }),
                    ..Default::default()
                }
            }
            Err(e) => {
                pinger.host = "1.1.1.1"
                    .parse::<IpAddr>()
                    .map_err(|e| JobServiceError::AgentIssue(AgentError::Ping(e.into()).into()))?;
                if pinger.ping(PingSequence(0), &[0; 8]).await.is_err() {
                    return Err(JobServiceError::AgentIssue(
                        AgentError::Ping(e.into()).into(),
                    ));
                } else {
                    JobResult {
                        id: job.id.clone(),
                        batch_id: job.batch_id.clone(),
                        run_id,
                        timestamp: Some(get_timestamp()?),
                        ping: Some(PingJobResult {
                            reachable: false,
                            ip_address: vec![],
                            latency: None,
                        }),
                        ..Default::default()
                    }
                }
            }
        };

        Ok(job_result)
    }
}

fn get_timestamp() -> Result<Timestamp, JobServiceError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| JobServiceError::AgentIssue(e.into()))?
        .to_proto())
}

#[cfg(test)]
mod tests {
    use gen_proto_types::job::{types::v1::PingJobType, v1::JobType};

    use super::*;

    /// Checks Ping Job with valid ip and listens for `reachable == true` with
    /// `ip_address == 1.0.0.1`
    ///
    /// Prefix is needed for nextest to exclude this test in the CI workflows,
    /// since there occur problems with permissions when running in Github
    /// Actions
    #[tokio::test]
    async fn no_ci_test_ping_success() {
        let job = Job {
            id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "1.0.0.1".to_string(),
            }),
            ..Default::default()
        };

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let ping_adapter = PingAdapter::new();
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                ping: Some(PingJobResult {
                    reachable: true,
                    ip_address: vec![1, 0, 0, 1],
                    latency: res.ping.as_ref().unwrap().latency
                }),
                ..Default::default()
            }
        )
    }

    /// Checks Ping Job with domain `one.one.one.one` and listens for `reachable
    /// == true` and an `ip_address` of either `1.0.0.1` or `1.1.1.1`
    ///
    /// Prefix is needed for nextest to exclude this test in the CI workflows,
    /// since there occur problems with permissions when running in Github
    /// Actions
    #[tokio::test]
    async fn no_ci_test_ping_success_domain() {
        let job = Job {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "one.one.one.one".to_string(),
            }),
            ..Default::default()
        };

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let ping_adapter = PingAdapter::new();
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        let expected_result_alt_1 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id: run_id.clone(),
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: vec![1, 0, 0, 1],
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };
        let expected_result_alt_2 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id,
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: vec![1, 1, 1, 1],
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };
        assert!(res == expected_result_alt_1 || res == expected_result_alt_2)
    }

    /// Checks for `reachable == false` for not reachable IPs
    ///
    /// Prefix is needed for nextest to exclude this test in the CI workflows,
    /// since there occur problems with permissions when running in Github
    /// Actions
    #[tokio::test]
    async fn no_ci_test_ping_error() {
        let job = Job {
            id: "CQybHx0FnQpv0SxRoVNou".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "169.254.0.0".to_string(),
            }),
            ..Default::default()
        };

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let ping_adapter = PingAdapter::new();
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "CQybHx0FnQpv0SxRoVNou".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                ping: Some(PingJobResult {
                    reachable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
    }
}
