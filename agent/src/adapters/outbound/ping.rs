use std::{
    net::IpAddr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use gen_proto_types::{
    job::v1::Job,
    job_result::{types::v1::PingJobResult, v1::JobResult},
};
use protocheck::{types, types::Timestamp};
use rand::random;
use surge_ping::{Client, Config, IcmpPacket, PingIdentifier, PingSequence};
use tracing::info;

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
    async fn execute(&self, job: &Job) -> Result<JobResult, JobServiceError> {
        let ping_details = job.ping.as_ref().unwrap();

        info!(
            "executing ping job with id {} for host {}",
            job.id, ping_details.host
        );

        let client = Client::new(&Config::default())
            .map_err(|e| JobServiceError::AgentIssue(AgentError::Ping(e.into()).into()))?;
        let mut pinger = client
            .pinger(
                ping_details
                    .host
                    .parse::<IpAddr>()
                    .map_err(|e| JobServiceError::AgentIssue(AgentError::Ping(e.into()).into()))?,
                PingIdentifier(random()),
            )
            .await;

        pinger.timeout(Duration::from_secs(1));

        let job_result = match pinger.ping(PingSequence(0), &[0; 8]).await {
            Ok((packet, duration)) => {
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
                    timestamp: Some(get_timestamp()?),
                    ping: Some(PingJobResult {
                        reachable: true,
                        ip_address: match packet {
                            IcmpPacket::V4(packet) => packet.get_real_dest().octets().to_vec(),
                            IcmpPacket::V6(packet) => packet.get_real_dest().octets().to_vec(),
                        },
                        latency: Some(types::Duration {
                            seconds: duration.as_secs() as i64,
                            nanos: 0,
                        }),
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
    Ok(Timestamp {
        seconds: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| JobServiceError::AgentIssue(e.into()))?
            .as_secs() as i64,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use gen_proto_types::job::{types::v1::PingJobType, v1::JobType};

    use super::*;

    #[tokio::test]
    async fn test_ping_success() {
        let job = Job {
            id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "1.0.0.1".to_string(),
            }),
            ..Default::default()
        };

        let ping_adapter = PingAdapter::new();
        let res = ping_adapter.execute(&job).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "cjz-BKp5cg6lsjMjYNz3R".to_string(),
                timestamp: get_current_timestamp(),
                ping: Some(PingJobResult {
                    reachable: true,
                    ip_address: vec![1, 0, 0, 1],
                    latency: res.ping.as_ref().unwrap().latency
                }),
                ..Default::default()
            }
        )
    }

    #[tokio::test]
    async fn test_ping_error() {
        let job = Job {
            id: "CQybHx0FnQpv0SxRoVNou".to_string(),
            job_type: JobType::Ping.into(),
            ping: Some(PingJobType {
                host: "169.254.0.0".to_string(),
            }),
            ..Default::default()
        };

        let ping_adapter = PingAdapter::new();
        let res = ping_adapter.execute(&job).await.unwrap();
        assert_eq!(
            res,
            JobResult {
                id: "CQybHx0FnQpv0SxRoVNou".to_string(),
                timestamp: get_current_timestamp(),
                ping: Some(PingJobResult {
                    reachable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
    }

    fn get_current_timestamp() -> Option<Timestamp> {
        Some(Timestamp {
            seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
            nanos: 0,
        })
    }
}
