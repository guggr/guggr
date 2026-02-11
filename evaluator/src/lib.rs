use std::time::Duration;

use deadpool_lapin::{Pool, Runtime};
use tokio::time::sleep;
use tracing::{error, info, warn};

pub mod adapters;
pub mod core;

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
