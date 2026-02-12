use std::{error::Error, net::IpAddr, time::Duration};

use deadpool_lapin::{Pool, Runtime};
use nanoid::nanoid;
use protocheck::types::Timestamp;
use tokio::{net::lookup_host, time::sleep};
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub trait ToProto<T> {
    fn to_proto(&self) -> T;
}

impl ToProto<protocheck::types::Duration> for Duration {
    fn to_proto(&self) -> protocheck::types::Duration {
        protocheck::types::Duration {
            // Theoretically u64 can be bigger than i64 but that only happens when working with
            // times very far in the future
            seconds: self.as_secs() as i64,
            // Nanos are always 0 <= n < 1_000_000_000 = 10^9. Therefore, casting is safe
            nanos: self.subsec_nanos() as i32,
        }
    }
}

impl ToProto<Timestamp> for Duration {
    fn to_proto(&self) -> Timestamp {
        Timestamp {
            // Theoretically u64 can be bigger than i64 but that only happens when working with
            // times very far in the future
            seconds: self.as_secs() as i64,
            // Nanos are always 0 <= n < 1_000_000_000 = 10^9. Therefore, casting is safe
            nanos: self.subsec_nanos() as i32,
        }
    }
}

/// Creates an RabbitMQ Pool via deadpool.
///
/// Errors when there is a problem with connecting to RabbitMQ.
pub async fn create_rabbitmq_pool(connection_url: &str) -> Result<Pool, Box<dyn Error>> {
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

/// Sets up tracing to log in JSON format
///
/// Panics if there is a problem with parsing the default EnvFilter.
pub fn init_tracing() {
    let fmt_layer = fmt::layer().with_file(true).with_line_number(true).json(); // Keep JSON for production logs

    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init(); // .init() is shorthand for set_global_default
}

/// Helps with resolving domains to IPs by utilizing tokio's `lookup_host`
/// method.
pub async fn resolve_domain(domain: String) -> Option<IpAddr> {
    // This is needed since lookup_host needs a port
    let domain = format!("{}:0", domain);
    lookup_host(domain)
        .await
        .ok()?
        .next()
        .map(|sock_addr| sock_addr.ip())
}

/// Generates a unique run id every time it is called. To be unique across
/// multiple agents the `nanoid` part is prefixed with the hostname.
/// If the hostname can not be retrieved, the `nanoid` is prefixed with
/// `agent-unknown`.
///
/// # Example
/// ```rust
/// use agent::generate_run_id;
///
/// println!("{}", generate_run_id());
/// ```
pub fn generate_run_id() -> String {
    match hostname::get() {
        Ok(hostname) => {
            let run_id = format!(
                "{}-{}",
                hostname.to_string_lossy().to_lowercase(),
                nanoid!()
            );
            run_id
        }
        Err(err) => {
            warn!("error getting hostname: {}", err);
            let run_id = format!("agent-unknown-{}", nanoid!());
            run_id
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resolve_domain_success() {
        let domain = String::from("one.one.one.one");
        let resolved = resolve_domain(domain).await;
        assert!(
            resolved == Some(IpAddr::from([1, 0, 0, 1]))
                || resolved == Some(IpAddr::from([1, 1, 1, 1])),
        );
    }

    #[tokio::test]
    async fn test_resolve_domain_failure() {
        let domain = String::from("parallel-dampfmaschine.info");
        assert_eq!(resolve_domain(domain).await, None);
    }
}
