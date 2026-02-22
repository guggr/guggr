mod basic;
mod connection_url;

mod agent;
mod api_service;
mod postgres;
mod rabbitmq;

use std::{env, num::ParseIntError};

pub use agent::AgentConfig;
pub use api_service::ApiServiceConfig;
pub use postgres::PostgresConfig;
pub use rabbitmq::RabbitMQConfig;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Error while loading config variable")]
    Env(#[from] env::VarError),
    #[error("Error while parsing integer")]
    ParseInt(#[from] ParseIntError),
}
