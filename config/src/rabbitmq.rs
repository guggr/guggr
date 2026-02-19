use std::{env, env::VarError};

use crate::{ConfigError, basic::BasicConfig, connection_url::ConnectionUrl};

#[derive(Debug, PartialEq)]
pub struct RabbitMQConfig {
    basic: BasicConfig,
    vhost: String,
    queue_names: Vec<String>,
}

impl RabbitMQConfig {
    pub fn from_env(queue_name_keys: &[&'static str]) -> Result<Self, ConfigError> {
        let user = env::var("RABBITMQ_USER")?;
        let password = env::var("RABBITMQ_PASSWORD")?;
        let host = env::var("RABBITMQ_HOST")?;
        let port = env::var("RABBITMQ_PORT")?;
        let vhost = match env::var("RABBITMQ_VHOST") {
            Ok(v) => v,
            Err(VarError::NotPresent) => "/".to_owned(),
            Err(e) => return Err(e.into()),
        };

        let mut queue_names = Vec::with_capacity(queue_name_keys.len());

        for key in queue_name_keys {
            queue_names.push(env::var(key)?);
        }

        Ok(Self {
            basic: BasicConfig::new(user, password, host, port),
            vhost,
            queue_names,
        })
    }

    pub fn queue_names(&self) -> Vec<String> {
        self.queue_names.clone()
    }

    pub fn queue_name(&self, idx: usize) -> Option<String> {
        self.queue_names.get(idx).cloned()
    }

    fn vhost(&self) -> String {
        self.vhost.clone()
    }

    fn protocol(&self, tls: bool) -> String {
        if tls {
            "amqps".to_owned()
        } else {
            "amqp".to_owned()
        }
    }

    pub fn connection_url(&self, tls: bool) -> String {
        self.basic
            .connection_url_builder(self.protocol(tls), self.vhost())
    }
}

#[cfg(test)]
mod tests {
    use std::{env::VarError, error::Error};

    use super::*;
    use crate::basic::BasicConfigTrait;

    #[test]
    fn missing_variables() {
        assert_eq!(
            RabbitMQConfig::from_env(&["test"])
                .unwrap_err()
                .source()
                .unwrap()
                .downcast_ref::<VarError>()
                .unwrap()
                .to_owned(),
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
                RabbitMQConfig::from_env(&["queue_a", "queue_b"])
                    .unwrap_err()
                    .source()
                    .unwrap()
                    .downcast_ref::<VarError>()
                    .unwrap()
                    .to_owned(),
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
            let config = RabbitMQConfig::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                RabbitMQConfig {
                    basic: BasicConfig::new(
                        "user".to_owned(),
                        "password".to_owned(),
                        "host".to_owned(),
                        "port".to_owned(),
                    ),
                    vhost: "/".to_owned(),

                    queue_names: vec!["a".to_owned(), "b".to_owned()],
                }
            );

            assert_eq!(config.basic.user(), "user");
            assert_eq!(config.basic.password(), "password");
            assert_eq!(config.basic.host(), "host");
            assert_eq!(config.basic.port(), "port");
            assert_eq!(config.vhost(), "/".to_owned());
            assert_eq!(
                config.connection_url(false),
                "amqp://user:password@host:port/%2F"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://user:password@host:port/%2F"
            );

            assert_eq!(config.queue_names(), vec!["a".to_owned(), "b".to_owned()]);
            assert_eq!(config.queue_name(0), Some("a".to_owned()));
            assert_eq!(config.queue_name(1), Some("b".to_owned()));
            assert_eq!(config.queue_name(2), None);
            assert_eq!(config.queue_name(3), None);
            assert_eq!(config.queue_name(99), None);
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
            let config = RabbitMQConfig::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                RabbitMQConfig {
                    basic: BasicConfig::new(
                        "user".to_owned(),
                        "password".to_owned(),
                        "host".to_owned(),
                        "port".to_owned(),
                    ),
                    vhost: "/".to_owned(),

                    queue_names: vec!["a".to_owned(), "b".to_owned()],
                }
            );

            assert_eq!(config.basic.user(), "user");
            assert_eq!(config.basic.password(), "password");
            assert_eq!(config.basic.host(), "host");
            assert_eq!(config.basic.port(), "port");
            assert_eq!(config.vhost(), "/");
            assert_eq!(
                config.connection_url(false),
                "amqp://user:password@host:port/%2F"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://user:password@host:port/%2F"
            );

            assert_eq!(config.queue_names(), vec!["a".to_owned(), "b".to_owned()]);
            assert_eq!(config.queue_name(0), Some("a".to_owned()));
            assert_eq!(config.queue_name(1), Some("b".to_owned()));
            assert_eq!(config.queue_name(2), None);
            assert_eq!(config.queue_name(3), None);
            assert_eq!(config.queue_name(99), None);
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
            let config = RabbitMQConfig::from_env(&["queue_a", "queue_b"]).unwrap();
            assert_eq!(
                config,
                RabbitMQConfig {
                    basic: BasicConfig::new(
                        "us:r".to_owned(),
                        "p@ssword".to_owned(),
                        "host".to_owned(),
                        "port".to_owned()
                    ),
                    vhost: "vh@st".to_owned(),

                    queue_names: vec!["a".to_owned(), "b".to_owned()],
                }
            );

            assert_eq!(config.basic.user(), "us:r");
            assert_eq!(config.basic.password(), "p@ssword");
            assert_eq!(config.basic.host(), "host");
            assert_eq!(config.basic.port(), "port");
            assert_eq!(config.vhost(), "vh@st".to_owned());
            assert_eq!(
                config.connection_url(false),
                "amqp://us%3Ar:p%40ssword@host:port/vh%40st"
            );
            assert_eq!(
                config.connection_url(true),
                "amqps://us%3Ar:p%40ssword@host:port/vh%40st"
            );

            assert_eq!(config.queue_names(), vec!["a".to_owned(), "b".to_owned()]);
            assert_eq!(config.queue_name(0), Some("a".to_owned()));
            assert_eq!(config.queue_name(1), Some("b".to_owned()));
            assert_eq!(config.queue_name(2), None);
            assert_eq!(config.queue_name(3), None);
            assert_eq!(config.queue_name(99), None);
        })
    }
}
