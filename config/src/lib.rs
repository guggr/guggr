use std::env;

use anyhow::{Context, Result};

pub struct Config {
    rabbitmq_user: String,
    rabbitmq_password: String,
    rabbitmq_host: String,
    rabbitmq_port: String,

    rabbitmq_queue_name: Vec<String>,
}

impl Config {
    pub fn from_env(queue_name_keys: &[&'static str]) -> Result<Self> {
        let user = env::var("RABBITMQ_USER").context("reading RABBITMQ_USER env var")?;
        let password =
            env::var("RABBITMQ_PASSWORD").context("reading RABBITMQ_PASSWORD env var")?;
        let host = env::var("RABBITMQ_HOST").context("reading RABBITMQ_HOST env var")?;
        let port = env::var("RABBITMQ_PORT").context("reading RABBITMQ_PORT env var")?;

        let mut queue_names = Vec::with_capacity(queue_name_keys.len());

        for key in queue_name_keys {
            queue_names.push(env::var(key).context("reading RABBITMQ_QUEUE_NAME env var")?);
        }

        Ok(Self {
            rabbitmq_user: user,
            rabbitmq_password: password,
            rabbitmq_host: host,
            rabbitmq_port: port,

            rabbitmq_queue_name: queue_names,
        })
    }

    pub fn rabbitmq_user(&self) -> String {
        self.rabbitmq_user.clone()
    }

    pub fn rabbitmq_password(&self) -> String {
        self.rabbitmq_password.clone()
    }

    pub fn rabbitmq_host(&self) -> String {
        self.rabbitmq_host.clone()
    }

    pub fn rabbitmq_port(&self) -> String {
        self.rabbitmq_port.clone()
    }
}
