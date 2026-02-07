use std::{
    net::IpAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::anyhow;
use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::PingJobResult, v1::JobResult},
};
use protocheck::{types, types::Timestamp};
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tracing::info;

use crate::core::{ports::monitor::MonitorPort, service::jobservice::JobServiceError};

pub struct PingAdapter {}

impl PingAdapter {
    pub fn new() -> Self {
        PingAdapter {}
    }
}

#[async_trait]
impl MonitorPort for PingAdapter {
    async fn execute(&self, job: &Job) -> anyhow::Result<JobResult, JobServiceError> {
        let ping_details = job.ping.as_ref().unwrap();

        info!(
            "executing ping job with id {} for host {}",
            job.id, ping_details.host
        );

        let client =
            Client::new(&Config::default()).map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?;
        let mut pinger = client
            .pinger(
                ping_details
                    .host
                    .parse::<IpAddr>()
                    .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?,
                PingIdentifier(random()),
            )
            .await;

        pinger.timeout(Duration::from_secs(1));

        let job_result = match pinger.ping(PingSequence(0), &[0; 8]).await {
            Ok((IcmpPacket::V4(packet), duration)) => {
                info!("received ping v4 from {}", ping_details.host);

                JobResult {
                    id: job.id.clone(),
                    timestamp: Some(get_timestamp()?),
                    http: None,
                    ping: Some(PingJobResult {
                        reachable: true,
                        ip_address: packet.get_real_dest().octets().to_vec(),
                        latency: Some(types::Duration {
                            seconds: duration.as_secs() as i64,
                            nanos: 0,
                        }),
                    }),
                }
            }
            Ok((IcmpPacket::V6(packet), duration)) => {
                info!("received ping v6 from {}", ping_details.host);

                JobResult {
                    id: job.id.clone(),
                    timestamp: Some(get_timestamp()?),
                    http: None,
                    ping: Some(PingJobResult {
                        reachable: true,
                        ip_address: packet.get_real_dest().octets().to_vec(),
                        latency: Some(types::Duration {
                            seconds: duration.as_secs() as i64,
                            nanos: 0,
                        }),
                    }),
                }
            }
            Err(e) => {
                pinger.host = "1.1.1.1"
                    .parse::<IpAddr>()
                    .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?;
                if pinger.ping(PingSequence(0), &[0; 8]).await.is_err() {
                    return Err(JobServiceError::AgentIssue(anyhow!(e)));
                } else {
                    JobResult {
                        id: job.id.clone(),
                        timestamp: Some(get_timestamp()?),
                        http: None,
                        ping: Some(PingJobResult {
                            reachable: false,
                            ip_address: vec![],
                            latency: None,
                        }),
                    }
                }
            }
        };

        Ok(job_result)
    }
}

fn get_timestamp() -> anyhow::Result<Timestamp, JobServiceError> {
    Ok(Timestamp {
        seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| JobServiceError::AgentIssue(anyhow!(e)))?
            .as_secs() as i64,
        ..Default::default()
    })
}
