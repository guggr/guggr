use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use tokio::time::{Interval, MissedTickBehavior};
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{debug, info};

use crate::core::{
    domain::errors::JobSchedulerError,
    ports::{periodic_task::PeriodicTask, ticker::Ticker},
};

/// Struct for Ticker adapter. Holds its own task tracker for implementing
/// graceful shutdown.
pub struct SchedulerTicker {
    task: Arc<dyn PeriodicTask>,
    interval: Interval,
    task_tracker: TaskTracker,
    shutdown_token: CancellationToken,
}

impl SchedulerTicker {
    /// Creates a new [`SchedulerTicker`] for running a periodic task. Requires
    /// a shutdown token for graceful shutdown.
    #[must_use]
    pub fn new(
        task: Arc<dyn PeriodicTask>,
        run_every: Duration,
        shutdown_token: CancellationToken,
    ) -> Self {
        Self {
            task,
            interval: tokio::time::interval(run_every),
            task_tracker: TaskTracker::new(),
            shutdown_token,
        }
    }
}

#[async_trait]
impl Ticker for SchedulerTicker {
    /// Starts the ticker with the defined interval.
    ///
    /// - Spawns a new tokio task for running the task in the task pool for
    ///   each tick.
    /// - Handles graceful shutdown via the passed [`CancellationToken`].
    #[allow(clippy::ignored_unit_patterns)]
    async fn start(self) {
        let mut interval = self.interval;
        interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let task = Arc::clone(&self.task);

                    debug!("triggered task run in own task");
                    self.task_tracker.spawn(async move {
                        match task.run().await {
                            Ok(()) => tracing::debug!("Task completed successfully."),

                            Err(JobSchedulerError::DatabaseUnavailable(e)) => {
                                // We use WARN here because it's a transient infra issue
                                tracing::warn!("Postgres is unreachable. Skipping this tick. {e}");
                            }

                            Err(JobSchedulerError::QueueUnavailable(e)) => {
                                // We use ERROR because jobs were fetched but couldn't be sent
                                tracing::error!("RabbitMQ is down. Failed to dispatch jobs! {e}");
                            }

                            Err(e) => {
                                tracing::error!("Critical/Unknown error in task: {e}");
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

        info!("Waiting for in-flight tasks to complete...");
        self.task_tracker.wait().await;

        info!("all tasks finished");
    }
}
