use std::{env, env::VarError};

use crate::{ConfigError, basic::BasicConfig, connection_url::ConnectionUrl};

#[derive(Debug, PartialEq, Eq)]
pub struct PostgresConfig {
    basic: BasicConfig,
    database: String,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let user = env::var("POSTGRES_USER")?;
        let password = env::var("POSTGRES_PASSWORD")?;
        let host = env::var("POSTGRES_HOST")?;
        let port = env::var("POSTGRES_PORT")?;
        let database = match env::var("POSTGRES_DATABASE") {
            Ok(v) => v,
            Err(VarError::NotPresent) => "guggr".to_owned(),
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            basic: BasicConfig::new(user, password, host, port),
            database,
        })
    }

    fn database(&self) -> String {
        self.database.clone()
    }

    #[must_use]
    pub fn connection_url(&self) -> String {
        self.basic
            .connection_url_builder("postgres".to_owned(), self.database())
    }
}

#[cfg(test)]
mod tests {
    use std::{env::VarError, error::Error};

    use super::*;
    use crate::basic::BasicConfigTrait;

    #[test]
    fn pg_missing_variables() {
        assert_eq!(
            PostgresConfig::from_env()
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
    fn pg_one_missing_variable() {
        let env_vars = vec![
            ("POSTGRES_USER", Some("user")),
            ("POSTGRES_PASSWORD", Some("password")),
            ("POSTGRES_PORT", Some("port")),
            ("POSTGRES_DATABASE", Some("database")),
        ];

        temp_env::with_vars(env_vars, || {
            assert_eq!(
                PostgresConfig::from_env()
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
    fn pg_valid_variables() {
        let env_vars = vec![
            ("POSTGRES_USER", Some("pg_user")),
            ("POSTGRES_PASSWORD", Some("pg_password")),
            ("POSTGRES_HOST", Some("pg_host")),
            ("POSTGRES_PORT", Some("pg_port")),
            ("POSTGRES_DATABASE", Some("database")),
        ];

        temp_env::with_vars(env_vars, || {
            let config = PostgresConfig::from_env().unwrap();
            assert_eq!(
                config,
                PostgresConfig {
                    basic: BasicConfig::new(
                        "pg_user".to_string(),
                        "pg_password".to_string(),
                        "pg_host".to_string(),
                        "pg_port".to_string()
                    ),
                    database: "database".to_string(),
                }
            );

            assert_eq!(config.basic.user(), "pg_user");
            assert_eq!(config.basic.password(), "pg_password");
            assert_eq!(config.basic.host(), "pg_host");
            assert_eq!(config.basic.port(), "pg_port");
            assert_eq!(config.database(), "database");
            assert_eq!(
                config.connection_url(),
                "postgres://pg_user:pg_password@pg_host:pg_port/database"
            );
        })
    }

    #[test]
    fn pg_special_characters() {
        let env_vars = vec![
            ("POSTGRES_USER", Some("pg_us:r")),
            ("POSTGRES_PASSWORD", Some("pg_p@ssword")),
            ("POSTGRES_HOST", Some("pg_host")),
            ("POSTGRES_PORT", Some("pg_port")),
            ("POSTGRES_DATABASE", Some("dat@base")),
        ];

        temp_env::with_vars(env_vars, || {
            let config = PostgresConfig::from_env().unwrap();
            assert_eq!(
                config,
                PostgresConfig {
                    basic: BasicConfig::new(
                        "pg_us:r".to_string(),
                        "pg_p@ssword".to_string(),
                        "pg_host".to_string(),
                        "pg_port".to_string()
                    ),
                    database: "dat@base".to_string(),
                }
            );

            assert_eq!(config.basic.user(), "pg_us:r");
            assert_eq!(config.basic.password(), "pg_p@ssword");
            assert_eq!(config.basic.host(), "pg_host");
            assert_eq!(config.basic.port(), "pg_port");
            assert_eq!(config.database(), "dat@base");
            assert_eq!(
                config.connection_url(),
                "postgres://pg_us%3Ar:pg_p%40ssword@pg_host:pg_port/dat%40base"
            );
        })
    }

    #[test]
    fn database_empty() {
        let env_vars = vec![
            ("POSTGRES_USER", Some("pg_user")),
            ("POSTGRES_PASSWORD", Some("pg_password")),
            ("POSTGRES_HOST", Some("pg_host")),
            ("POSTGRES_PORT", Some("pg_port")),
            ("POSTGRES_DATABASE", None),
        ];

        temp_env::with_vars(env_vars, || {
            let config = PostgresConfig::from_env().unwrap();
            assert_eq!(
                config,
                PostgresConfig {
                    basic: BasicConfig::new(
                        "pg_user".to_string(),
                        "pg_password".to_string(),
                        "pg_host".to_string(),
                        "pg_port".to_string()
                    ),
                    database: "guggr".to_string(),
                }
            );

            assert_eq!(config.basic.user(), "pg_user");
            assert_eq!(config.basic.password(), "pg_password");
            assert_eq!(config.basic.host(), "pg_host");
            assert_eq!(config.basic.port(), "pg_port");
            assert_eq!(config.database(), "guggr");
            assert_eq!(
                config.connection_url(),
                "postgres://pg_user:pg_password@pg_host:pg_port/guggr"
            );
        })
    }
}
