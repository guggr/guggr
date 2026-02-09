use std::time::Duration;

use lapin::{Connection, ConnectionProperties};
use tokio::time::sleep;
use tracing::{error, info, warn};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub async fn connect_rabbitmq(
    connection_url: String,
) -> Result<Connection, Box<dyn std::error::Error>> {
    let mut retry_count = 0;

    let connection = loop {
        match Connection::connect(&connection_url, ConnectionProperties::default()).await {
            Ok(conn) => {
                info!("successfully connected to rabbitmq host");
                break conn;
            }
            Err(e) => {
                retry_count += 1;
                if retry_count > 5 {
                    error!("error connecting to rabbitmq after 5 retries: {}", e);
                    std::process::exit(1);
                }

                warn!(
                    "temporary error connecting to rabbitmq (try {}/5). retrying...",
                    retry_count
                );
                sleep(Duration::from_secs(10)).await;
            }
        }
    };

    Ok(connection)
}

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
