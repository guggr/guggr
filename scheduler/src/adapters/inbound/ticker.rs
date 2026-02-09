use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::time::{Interval, MissedTickBehavior};

use crate::core::{
    domain::errors::JobSchedulerError,
    ports::ticker::Ticker,
    service::{self, schedulerservice::SchedulerService},
};

pub struct SchedulerTicker {
    service: Arc<SchedulerService>,
    interval: Interval,
}

impl SchedulerTicker {
    pub fn new(service: Arc<SchedulerService>, run_every: Duration) -> Self {
        Self {
            service,
            interval: tokio::time::interval(run_every),
        }
    }
}

#[async_trait]
impl Ticker for SchedulerTicker {
    async fn start(self) {
        let mut interval = self.interval;
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            interval.tick().await;
            let service = Arc::clone(&self.service);

            tokio::spawn(async move {
                match service.run().await {
                    Ok(_) => tracing::debug!("Batch processed successfully."),

                    Err(JobSchedulerError::DatabaseUnavailable) => {
                        // We use WARN here because it's a transient infra issue
                        tracing::warn!("Postgres is unreachable. Skipping this tick.");
                    }

                    Err(JobSchedulerError::QueueUnavailable(e)) => {
                        // We use ERROR because jobs were fetched but couldn't be sent
                        tracing::error!("RabbitMQ is down. Failed to dispatch jobs! {e}");
                    }

                    Err(e) => {
                        tracing::error!("Critical/Unknown error in service: {e}");
                    }
                }
            });
        }
    }
}
