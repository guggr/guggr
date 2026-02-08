use std::{env, env::VarError};

use urlencoding::encode;

use crate::ConfigError;

#[derive(Debug, PartialEq)]
pub struct PostgresConfig {
    postgres_user: String,
    postgres_password: String,
    postgres_host: String,
    postgres_port: String,
    postgres_database: String,
}

impl PostgresConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let postgres_user = env::var("POSTGRES_USER")?;
        let postgres_password = env::var("POSTGRES_PASSWORD")?;
        let postgres_host = env::var("POSTGRES_HOST")?;
        let postgres_port = env::var("POSTGRES_PORT")?;
        let postgres_database = match env::var("POSTGRES_DATABASE") {
            Ok(v) => v,
            Err(VarError::NotPresent) => "guggr".to_owned(),
            Err(e) => return Err(e.into()),
        };

        Ok(Self {
            postgres_user,
            postgres_password,
            postgres_host,
            postgres_port,
            postgres_database,
        })
    }

    pub fn postgres_user(&self) -> String {
        self.postgres_user.clone()
    }

    pub fn postgres_password(&self) -> String {
        self.postgres_password.clone()
    }

    pub fn postgres_host(&self) -> String {
        self.postgres_host.clone()
    }

    pub fn postgres_port(&self) -> String {
        self.postgres_port.clone()
    }

    pub fn postgres_database(&self) -> String {
        self.postgres_database.clone()
    }

    pub fn postgres_connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            encode(&self.postgres_user()),
            encode(&self.postgres_password()),
            &self.postgres_host(),
            &self.postgres_port(),
            encode(&self.postgres_database())
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{env::VarError, error::Error};

    use crate::PostgresConfig;

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
                    postgres_user: "pg_user".to_string(),
                    postgres_password: "pg_password".to_string(),
                    postgres_host: "pg_host".to_string(),
                    postgres_port: "pg_port".to_string(),
                    postgres_database: "database".to_string(),
                }
            );

            assert_eq!(config.postgres_user(), "pg_user");
            assert_eq!(config.postgres_password(), "pg_password");
            assert_eq!(config.postgres_host(), "pg_host");
            assert_eq!(config.postgres_port(), "pg_port");
            assert_eq!(config.postgres_database(), "database");
            assert_eq!(
                config.postgres_connection_url(),
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
                    postgres_user: "pg_us:r".to_string(),
                    postgres_password: "pg_p@ssword".to_string(),
                    postgres_host: "pg_host".to_string(),
                    postgres_port: "pg_port".to_string(),
                    postgres_database: "dat@base".to_string(),
                }
            );

            assert_eq!(config.postgres_user(), "pg_us:r");
            assert_eq!(config.postgres_password(), "pg_p@ssword");
            assert_eq!(config.postgres_host(), "pg_host");
            assert_eq!(config.postgres_port(), "pg_port");
            assert_eq!(config.postgres_database(), "dat@base");
            assert_eq!(
                config.postgres_connection_url(),
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
                    postgres_user: "pg_user".to_string(),
                    postgres_password: "pg_password".to_string(),
                    postgres_host: "pg_host".to_string(),
                    postgres_port: "pg_port".to_string(),
                    postgres_database: "guggr".to_string(),
                }
            );

            assert_eq!(config.postgres_user(), "pg_user");
            assert_eq!(config.postgres_password(), "pg_password");
            assert_eq!(config.postgres_host(), "pg_host");
            assert_eq!(config.postgres_port(), "pg_port");
            assert_eq!(config.postgres_database(), "guggr");
            assert_eq!(
                config.postgres_connection_url(),
                "postgres://pg_user:pg_password@pg_host:pg_port/guggr"
            );
        })
    }
}
