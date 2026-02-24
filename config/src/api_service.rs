use std::env;

use crate::ConfigError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiServiceConfig {
    // can be an IP or Hostname
    host: String,
    port: u16,
    auth_ttl: i64,
    auth_refresh_ttl: i64,
    auth_secret: String,
}

impl ApiServiceConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let host = env::var("API_SERVICE_HOST")?;
        let port: u16 = env::var("API_SERVICE_PORT")?.parse()?;
        let auth_ttl: i64 = env::var("API_SERVICE_AUTH_TTL")?.parse()?;
        let auth_refresh_ttl: i64 = env::var("API_SERVICE_AUTH_REFRESH_TTL")?.parse()?;
        let auth_secret = env::var("API_SERVICE_AUTH_SECRET")?;
        Ok(Self {
            host,
            port,
            auth_ttl,
            auth_refresh_ttl,
            auth_secret,
        })
    }

    pub fn auth_ttl(&self) -> i64 {
        self.auth_ttl
    }

    pub fn auth_refresh_ttl(&self) -> i64 {
        self.auth_refresh_ttl
    }

    pub fn bind_address(&self) -> (&str, u16) {
        (&self.host, self.port)
    }

    pub fn auth_secret(&self) -> Vec<u8> {
        self.auth_secret.as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn api_valid_vars() {
        let env_vars = vec![
            ("API_SERVICE_HOST", Some("localhost")),
            ("API_SERVICE_PORT", Some("8000")),
            ("API_SERVICE_AUTH_TTL", Some("3600")),
            ("API_SERVICE_AUTH_REFRESH_TTL", Some("604800")),
            ("API_SERVICE_AUTH_SECRET", Some("very-secret")),
        ];
        temp_env::with_vars(env_vars, || {
            let config = ApiServiceConfig::from_env().unwrap();
            assert_eq!(
                config,
                ApiServiceConfig {
                    host: "localhost".to_string(),
                    port: 8000,
                    auth_ttl: 3600,
                    auth_refresh_ttl: 604800,
                    auth_secret: "very-secret".to_string()
                }
            );
            assert_eq!(config.bind_address(), ("localhost", 8000));
            assert_eq!(config.auth_ttl(), 3600);
            assert_eq!(config.auth_refresh_ttl(), 604800);
        })
    }

    #[test]
    fn invalid_port() {
        let env_vars = vec![
            ("API_SERVICE_HOST", Some("localhost")),
            ("API_SERVICE_PORT", Some("80000")),
            ("API_SERVICE_AUTH_TTL", Some("3600")),
            ("API_SERVICE_AUTH_REFRESH_TTL", Some("604800")),
            ("API_SERVICE_AUTH_SECRET", Some("very-secret")),
        ];
        temp_env::with_vars(env_vars, || {
            assert!(matches!(
                ApiServiceConfig::from_env().unwrap_err(),
                ConfigError::ParseInt(_)
            ))
        })
    }
}
