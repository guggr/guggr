use std::env;

#[derive(Debug, PartialEq)]
pub struct AgentConfig {
    ping_backup_endpoint: Option<String>,
    http_backup_endpoint: Option<String>,
}

impl AgentConfig {
    pub fn from_env() -> Self {
        let ping_backup_endpoint = env::var("AGENT_PING_BACKUP_ENDPOINT").ok();
        let http_backup_endpoint = env::var("AGENT_HTTP_BACKUP_ENDPOINT").ok();

        Self {
            ping_backup_endpoint,
            http_backup_endpoint,
        }
    }

    pub fn ping_backup_endpoint(&self) -> &Option<String> {
        &self.ping_backup_endpoint
    }

    pub fn http_backup_endpoint(&self) -> &Option<String> {
        &self.http_backup_endpoint
    }
}

#[cfg(test)]
mod tests {

    use crate::AgentConfig;

    #[test]
    fn agent_missing_variables() {
        assert_eq!(
            AgentConfig::from_env(),
            AgentConfig {
                ping_backup_endpoint: None,
                http_backup_endpoint: None,
            }
        )
    }

    #[test]
    fn agent_missing_http_backup_endpoint() {
        let env_vars = vec![("AGENT_PING_BACKUP_ENDPOINT", Some("1.1.1.1"))];

        temp_env::with_vars(env_vars, || {
            assert_eq!(
                AgentConfig::from_env(),
                AgentConfig {
                    ping_backup_endpoint: Some("1.1.1.1".to_string()),
                    http_backup_endpoint: None,
                }
            )
        })
    }

    #[test]
    fn agent_missing_ping_backup_endpoint() {
        let env_vars = vec![("AGENT_HTTP_BACKUP_ENDPOINT", Some("gug.gr"))];

        temp_env::with_vars(env_vars, || {
            assert_eq!(
                AgentConfig::from_env(),
                AgentConfig {
                    ping_backup_endpoint: None,
                    http_backup_endpoint: Some(String::from("gug.gr")),
                }
            )
        })
    }

    #[test]
    fn agent_valid_configuration() {
        let env_vars = vec![
            ("AGENT_PING_BACKUP_ENDPOINT", Some("1.1.1.1")),
            ("AGENT_HTTP_BACKUP_ENDPOINT", Some("gug.gr")),
        ];

        temp_env::with_vars(env_vars, || {
            assert_eq!(
                AgentConfig::from_env(),
                AgentConfig {
                    ping_backup_endpoint: Some("1.1.1.1".to_string()),
                    http_backup_endpoint: Some(String::from("gug.gr")),
                }
            );
        })
    }
}
