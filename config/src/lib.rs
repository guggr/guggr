mod basic;
mod connection_url;

mod agent;
mod postgres;
mod rabbitmq;

use std::env;

pub use agent::AgentConfig;
pub use postgres::PostgresConfig;
pub use rabbitmq::RabbitMQConfig;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Error while loading config variable")]
pub struct ConfigError(#[from] env::VarError);
