use std::{
    net::IpAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use agent::{ToProto, resolve_domain};
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::PingJobResult, v1::JobResult},
    job_types::v1::JobType,
};
use protify::proto_types::Timestamp;
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tracing::{debug, error, info, trace};

use crate::core::{
    ports::monitor::MonitorPort,
    service::jobservice::{AgentError, JobServiceError},
};

pub struct PingAdapter {
    backup_endpoint: Option<String>,
}

impl PingAdapter {
    pub const fn new(backup_endpoint: Option<String>) -> Self {
        Self { backup_endpoint }
    }
}

#[async_trait]
impl MonitorPort for PingAdapter {
    /// Executes a provided Job with
    /// [`gen_proto_types::job::types::v1::PingJobType`]. If the destination
    /// is not reachable, it optionally checks whether a configured "backup ip"
    /// is reachable.
    ///
    /// # Errors
    /// Returns a [`JobServiceError`] if there is a problem with
    /// - creating a Ping [`Client`]
    /// - resolving a given domain
    /// - parsing an IP address
    /// - reaching the "backup ip"
    /// - getting the current timestamp
    async fn execute(&self, job: &Job, run_id: String) -> Result<JobResult, JobServiceError> {
        let ping_details = job.ping.as_ref().unwrap();

        info!(
            "executing ping job with id {} for host {}",
            job.id, ping_details.host
        );

        // Check if domain provided or IP
        let host_ip = get_pinger_host(ping_details.host.clone()).await?;

        let mut config_builder = Config::builder();

        // Adjust config if IPv6 is provided
        if host_ip.is_ipv6() {
            trace!("received ipv6 address: {}", host_ip);
            config_builder = config_builder.kind(surge_ping::ICMP::V6);
        }

        let config = config_builder.build();
        let client = Client::new(&config)
            .map_err(|e| JobServiceError::AgentIssue(AgentError::Ping(e.into()).into()))?;

        let mut pinger = client.pinger(host_ip, PingIdentifier(random())).await;

        pinger.timeout(Duration::from_secs(1));

        let ping_result = match pinger.ping(PingSequence(0), &[0; 8]).await {
            Ok((packet, latency)) => {
                info!(
                    "received ping {} from {}",
                    match packet {
                        IcmpPacket::V4(_) => "v4",
                        IcmpPacket::V6(_) => "v6",
                    },
                    ping_details.host
                );

                PingJobResult {
                    reachable: true,
                    ip_address: match packet {
                        IcmpPacket::V4(packet) => packet.get_real_dest().octets().to_vec().into(),
                        IcmpPacket::V6(packet) => packet.get_real_dest().octets().to_vec().into(),
                    },
                    latency: Some(latency.to_proto()),
                }
            }
            Err(e) => {
                debug!("error while pinging endpoint: {}", e);
                if self.backup_endpoint.is_some() {
                    pinger.host = get_pinger_host(ping_details.host.clone()).await?;
                    if pinger.ping(PingSequence(0), &[0; 8]).await.is_err() {
                        return Err(JobServiceError::AgentIssue(
                            AgentError::Ping(e.into()).into(),
                        ));
                    }
                }

                PingJobResult {
                    reachable: false,
                    ip_address: vec![].into(),
                    latency: None,
                }
            }
        };

        Ok(JobResult {
            id: job.id.clone(),
            batch_id: job.batch_id.clone(),
            run_id,
            job_type: JobType::Ping.into(),
            timestamp: Some(get_timestamp()?),
            ping: Some(ping_result),
            ..Default::default()
        })
    }
}

fn get_timestamp() -> Result<Timestamp, JobServiceError> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| JobServiceError::AgentIssue(e.into()))?
        .to_proto())
}

async fn get_pinger_host(host: String) -> Result<IpAddr, JobServiceError> {
    if let Ok(ip) = host.parse::<IpAddr>() {
        Ok(ip)
    } else {
        debug!("ping adapter received domain. trying to resolve it...");
        if let Some(ip) = resolve_domain(host.clone()).await {
            Ok(ip)
        } else {
            error!("ping adapter could not resolve domain {}", host);
            Err(JobServiceError::AgentIssue(
                "could not resolve domain for ping job".into(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv6Addr;

    use agent::init_tracing;
    use gen_proto_types::{job::types::v1::PingJobType, job_types::v1::JobType};

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

        let ping_adapter = PingAdapter::new(None);
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                job_type: JobType::Ping.into(),
                ping: Some(PingJobResult {
                    reachable: true,
                    ip_address: vec![1, 0, 0, 1].into(),
                    latency: res.ping.as_ref().unwrap().latency
                }),
                ..Default::default()
            }
        )
    }

    /// Checks Ping Job with valid ipv6 address and listens for `reachable ==
    /// true` with `ip_address == 2606:4700:4700::1001`
    ///
    /// Prefix is needed for nextest to exclude this test in the CI workflows,
    /// since there occur problems with permissions when running in Github
    /// Actions
    #[tokio::test]
    async fn no_ci_test_ping_success_ipv6() {
        let job = Job {
            id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "::1".to_string(),
            }),
            ..Default::default()
        };

        init_tracing();

        let run_id = "agent-test-xutjQ15iP2MsMEuVfhQng".to_string();

        let ping_adapter = PingAdapter::new(None);
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                job_type: JobType::Ping.into(),
                ping: Some(PingJobResult {
                    reachable: true,
                    ip_address: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1].into(),
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

        let ping_adapter = PingAdapter::new(None);
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        let expected_result_alt_1 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id: run_id.clone(),
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            job_type: JobType::Ping.into(),
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: vec![1, 0, 0, 1].into(),
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };
        let expected_result_alt_2 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id: run_id.clone(),
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            job_type: JobType::Ping.into(),
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: vec![1, 1, 1, 1].into(),
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };

        let first_ipv6_addr = "2606:4700:4700::1001".parse::<Ipv6Addr>().unwrap();
        let expected_result_alt_3 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id: run_id.clone(),
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            job_type: JobType::Ping.into(),
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: first_ipv6_addr.octets().to_vec().into(),
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };

        let second_ipv6_addr = "2606:4700:4700::1111".parse::<Ipv6Addr>().unwrap();
        let expected_result_alt_4 = JobResult {
            id: "lNhirp0h2nBY0Xb6BMT1B".to_string(),
            batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
            run_id: run_id.clone(),
            // Needed since timestamps would be too accurate
            timestamp: res.timestamp,
            job_type: JobType::Ping.into(),
            ping: Some(PingJobResult {
                reachable: true,
                ip_address: second_ipv6_addr.octets().to_vec().into(),
                latency: res.ping.as_ref().unwrap().latency,
            }),
            ..Default::default()
        };

        dbg!(&res);

        assert!(
            res == expected_result_alt_1
                || res == expected_result_alt_2
                || res == expected_result_alt_3
                || res == expected_result_alt_4
        )
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

        let ping_adapter = PingAdapter::new(None);
        let res = ping_adapter.execute(&job, run_id.clone()).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "CQybHx0FnQpv0SxRoVNou".to_string(),
                batch_id: "slaXBvDDWLYFPkQ7wN0mb".to_string(),
                run_id,
                // Needed since timestamps would be too accurate
                timestamp: res.timestamp,
                job_type: JobType::Ping.into(),
                ping: Some(PingJobResult {
                    reachable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
    }
}
