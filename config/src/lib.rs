use std::env::{self, VarError};

use anyhow::{Context, Result};
use urlencoding::encode;

#[derive(Debug, PartialEq)]
pub struct Config {
    rabbitmq_user: String,
    rabbitmq_password: String,
    rabbitmq_host: String,
    rabbitmq_port: String,
    rabbitmq_vhost: Option<String>,

    rabbitmq_queue_names: Vec<String>,
}

impl Config {
    pub fn from_env(queue_name_keys: &[&'static str]) -> Result<Self> {
        let user = env::var("RABBITMQ_USER").context("reading RABBITMQ_USER env var")?;
        let password =
            env::var("RABBITMQ_PASSWORD").context("reading RABBITMQ_PASSWORD env var")?;
        let host = env::var("RABBITMQ_HOST").context("reading RABBITMQ_HOST env var")?;
        let port = env::var("RABBITMQ_PORT").context("reading RABBITMQ_PORT env var")?;
        let vhost = match env::var("RABBITMQ_VHOST") {
            Ok(v) => Some(v),
            Err(VarError::NotPresent) => None,
            Err(e) => return Err(e).context("reading RABBITMQ_VHOST env var"),
        };

        let mut queue_names = Vec::with_capacity(queue_name_keys.len());

        for key in queue_name_keys {
            queue_names.push(env::var(key).context("reading RABBITMQ_QUEUE_NAME env var")?);
        }

        Ok(Self {
            rabbitmq_user: user,
            rabbitmq_password: password,
            rabbitmq_host: host,
            rabbitmq_port: port,
            rabbitmq_vhost: vhost,

            rabbitmq_queue_names: queue_names,
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

    pub fn rabbitmq_vhost(&self) -> Option<String> {
        self.rabbitmq_vhost.clone()
    }

    pub fn connection_url(&self, tls: bool) -> String {
        let protocol = if tls { "amqps" } else { "amqp" };
        let encoded_vhost = match self.rabbitmq_vhost.as_deref() {
            None | Some("/") => "%2f".to_string(), // Treat both as the default vhost
            Some(v) => encode(v).to_string(),
        };

        format!(
            "{}://{}:{}@{}:{}/{}",
            protocol,
            encode(&self.rabbitmq_user()),
            encode(&self.rabbitmq_password()),
            self.rabbitmq_host(),
            self.rabbitmq_port(),
            encoded_vhost
        )
    }

    pub fn rabbitmq_queue_names(&self) -> Vec<String> {
        self.rabbitmq_queue_names.clone()
    }

    pub fn rabbitmq_queue_name(&self, idx: usize) -> Option<String> {
        self.rabbitmq_queue_names.get(idx).cloned()
    }
}

#[cfg(test)]
mod tests {
    use std::env::VarError;

    use super::*;

    #[test]
    fn missing_variables() {
        assert_eq!(
            Config::from_env(&["test"])
                .unwrap_err()
                .downcast::<VarError>()
                .unwrap(),
            VarError::NotPresent
        )
    }

    #[test]
    fn one_missing_variable() {
        let env_vars = vec![
            ("RABBITMQ_USER", Some("user")),
            ("RABBITMQ_PASSWORD", Some("password")),
            ("RABBITMQ_HOST", Some("host")),
            ("RABBITMQ_PORT", Some("port")),
            ("queue_a", Some("a")),
        ];

        temp_env::with_vars(env_vars, || {
            assert_eq!(
                Config::from_env(&["queue_a", "queue_b"])
                    .unwrap_err()
                    .downcast::<VarError>()
                    .unwrap(),
                VarError::NotPresent
            )
        })
    }

    #[test]
    fn valid_variables() {
        let env_vars = vec![
            ("RABBITMQ_USER", Some("user")),
            ("RABBITMQ_PASSWORD", Some("password")),
            ("RABBITMQ_HOST", Some("host")),
            ("RABBITMQ_PORT", Some("port")),
            ("RABBITMQ_VHOST", Some("/")),
            ("queue_a", Some("a")),
            ("queue_b", Some("b")),
        ];

        temp_env::with_vars(env_vars, || {
            let config = Config::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                Config {
                    rabbitmq_user: "user".to_owned(),
                    rabbitmq_password: "password".to_owned(),
                    rabbitmq_host: "host".to_owned(),
                    rabbitmq_port: "port".to_owned(),
                    rabbitmq_vhost: Some("/".to_owned()),

                    rabbitmq_queue_names: vec!["a".to_owned(), "b".to_owned()]
                }
            );

            assert_eq!(config.rabbitmq_user(), "user");
            assert_eq!(config.rabbitmq_password(), "password");
            assert_eq!(config.rabbitmq_host(), "host");
            assert_eq!(config.rabbitmq_port(), "port");
            assert_eq!(config.rabbitmq_vhost(), Some("/".to_owned()));
            assert_eq!(
                config.connection_url(false),
                "amqp://user:password@host:port/%2f"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://user:password@host:port/%2f"
            );

            assert_eq!(
                config.rabbitmq_queue_names(),
                vec!["a".to_owned(), "b".to_owned()]
            );
            assert_eq!(config.rabbitmq_queue_name(0), Some("a".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(1), Some("b".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(2), None);
            assert_eq!(config.rabbitmq_queue_name(3), None);
            assert_eq!(config.rabbitmq_queue_name(99), None);
        })
    }

    #[test]
    fn vhost_empty() {
        let env_vars = vec![
            ("RABBITMQ_USER", Some("user")),
            ("RABBITMQ_PASSWORD", Some("password")),
            ("RABBITMQ_HOST", Some("host")),
            ("RABBITMQ_PORT", Some("port")),
            ("RABBITMQ_VHOST", None),
            ("queue_a", Some("a")),
            ("queue_b", Some("b")),
        ];

        temp_env::with_vars(env_vars, || {
            let config = Config::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                Config {
                    rabbitmq_user: "user".to_owned(),
                    rabbitmq_password: "password".to_owned(),
                    rabbitmq_host: "host".to_owned(),
                    rabbitmq_port: "port".to_owned(),
                    rabbitmq_vhost: None,

                    rabbitmq_queue_names: vec!["a".to_owned(), "b".to_owned()]
                }
            );

            assert_eq!(config.rabbitmq_user(), "user");
            assert_eq!(config.rabbitmq_password(), "password");
            assert_eq!(config.rabbitmq_host(), "host");
            assert_eq!(config.rabbitmq_port(), "port");
            assert_eq!(config.rabbitmq_vhost(), None);
            assert_eq!(
                config.connection_url(false),
                "amqp://user:password@host:port/%2f"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://user:password@host:port/%2f"
            );

            assert_eq!(
                config.rabbitmq_queue_names(),
                vec!["a".to_owned(), "b".to_owned()]
            );
            assert_eq!(config.rabbitmq_queue_name(0), Some("a".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(1), Some("b".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(2), None);
            assert_eq!(config.rabbitmq_queue_name(3), None);
            assert_eq!(config.rabbitmq_queue_name(99), None);
        })
    }

    #[test]
    fn special_characters() {
        let env_vars = vec![
            ("RABBITMQ_USER", Some("us:r")),
            ("RABBITMQ_PASSWORD", Some("p@ssword")),
            ("RABBITMQ_HOST", Some("host")),
            ("RABBITMQ_PORT", Some("port")),
            ("RABBITMQ_VHOST", Some("vh@st")),
            ("queue_a", Some("a")),
            ("queue_b", Some("b")),
        ];

        temp_env::with_vars(env_vars, || {
            let config = Config::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                Config {
                    rabbitmq_user: "us:r".to_owned(),
                    rabbitmq_password: "p@ssword".to_owned(),
                    rabbitmq_host: "host".to_owned(),
                    rabbitmq_port: "port".to_owned(),
                    rabbitmq_vhost: Some("vh@st".to_owned()),

                    rabbitmq_queue_names: vec!["a".to_owned(), "b".to_owned()]
                }
            );

            assert_eq!(config.rabbitmq_user(), "us:r");
            assert_eq!(config.rabbitmq_password(), "p@ssword");
            assert_eq!(config.rabbitmq_host(), "host");
            assert_eq!(config.rabbitmq_port(), "port");
            assert_eq!(config.rabbitmq_vhost(), Some("vh@st".to_owned()));
            assert_eq!(
                config.connection_url(false),
                "amqp://us%3Ar:p%40ssword@host:port/vh%40st"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://us%3Ar:p%40ssword@host:port/vh%40st"
            );

            assert_eq!(
                config.rabbitmq_queue_names(),
                vec!["a".to_owned(), "b".to_owned()]
            );
            assert_eq!(config.rabbitmq_queue_name(0), Some("a".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(1), Some("b".to_owned()));
            assert_eq!(config.rabbitmq_queue_name(2), None);
            assert_eq!(config.rabbitmq_queue_name(3), None);
            assert_eq!(config.rabbitmq_queue_name(99), None);
        })
    }
}
