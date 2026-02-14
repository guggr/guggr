use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::Duration,
};

use chrono::{DateTime, NaiveDateTime};
use deadpool_lapin::{Pool, Runtime};
use ipnet::IpNet;
use protocheck::types::Timestamp;
use tokio::time::sleep;
use tracing::{error, info, warn};

pub mod adapters;
pub mod core;
pub mod telemetry;

/// # Errors
///
/// Exits after 5 pool creation tries with 10 seconds between each try
pub async fn create_rabbitmq_pool(connection_url: &str) -> Result<Pool, ()> {
    let config = deadpool_lapin::Config {
        url: Some(connection_url.into()),
        ..Default::default()
    };

    let mut retry_count = 0;

    let pool = loop {
        match config.create_pool(Some(Runtime::Tokio1)) {
            Ok(conn) => {
                info!("successfully created rabbitmq pool");
                break conn;
            }
            Err(e) => {
                retry_count += 1;
                if retry_count > 5 {
                    error!("error creating rabbitmq pool after 5 retries: {}", e);
                    std::process::exit(1);
                }

                warn!(
                    "temporary error creating rabbitmq pool (try {}/5). retrying...",
                    retry_count
                );
                sleep(Duration::from_secs(10)).await;
            }
        }
    };

    Ok(pool)
}

/// # Errors
///
/// Will return `Err` if the supplied byte slice is neither 4 nor 16 bytes
/// long
pub fn ipnet_from_bytes_host(bytes: &[u8]) -> Result<IpNet, &'static str> {
    match bytes.len() {
        4 => Ok(IpNet::from(IpAddr::V4(Ipv4Addr::new(
            bytes[0], bytes[1], bytes[2], bytes[3],
        )))),
        16 => {
            let arr: [u8; 16] = bytes.try_into().map_err(|_| "bad ipv6 length")?;
            Ok(IpNet::from(IpAddr::V6(Ipv6Addr::from(arr))))
        }
        _ => Err("ip bytes must be 4 (v4) or 16 (v6)"),
    }
}

/// # Errors
///
/// Will return `Err` if the supplied timestamp is larger than an i32
fn protocheck_duration_to_i32_millis(d: protocheck::types::Duration) -> Result<i32, &'static str> {
    let secs: i128 = i128::from(d.seconds);
    let nanos: i128 = i128::from(d.nanos);

    let ms: i128 = secs.saturating_mul(1_000) + nanos / 1_000_000;

    if ms < i128::from(i32::MIN) {
        return Err("duration too small for i32 milliseconds");
    }
    i32::try_from(ms).map_err(|_| "duration too large for i32 milliseconds")
}

fn naive_from_proto_ts(ts: &Timestamp) -> Option<NaiveDateTime> {
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32).map(|dt| dt.naive_local())
}
