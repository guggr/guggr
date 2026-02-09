use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::time::{Interval, MissedTickBehavior};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, info};

use crate::core::{
    domain::errors::JobSchedulerError, ports::ticker::Ticker,
    service::schedulerservice::SchedulerService,
};

pub struct SchedulerTicker {
    service: Arc<SchedulerService>,
    interval: Interval,
    task_tracker: TaskTracker,
    shutdown_token: CancellationToken,
}

impl SchedulerTicker {
    #[must_use]
    pub fn new(
        service: Arc<SchedulerService>,
        run_every: Duration,
        shutdown_token: CancellationToken,
    ) -> Self {
        Self {
            service,
            interval: tokio::time::interval(run_every),
            task_tracker: TaskTracker::new(),
            shutdown_token,
        }
    }
}

#[async_trait]
impl Ticker for SchedulerTicker {
    #[allow(clippy::ignored_unit_patterns)]
    async fn start(self) {
        let mut interval = self.interval;
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let service = Arc::clone(&self.service);

                    debug!("triggered service run in own task");
                    self.task_tracker.spawn(async move {
                        match service.run().await {
                            Ok(()) => tracing::debug!("Batch processed successfully."),

                            Err(JobSchedulerError::DatabaseUnavailable(e)) => {
                                // We use WARN here because it's a transient infra issue
                                tracing::warn!("Postgres is unreachable. Skipping this tick. {e}");
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
                _ = self.shutdown_token.cancelled() => {
                    info!("shutdown token cancelled, exiting ticker");
                    break;
                }

            }
        }

        self.task_tracker.close();

        info!("Waiting for in-flight jobs to complete...");
        self.task_tracker.wait().await;

        info!("all jobs finished");
    }
}
