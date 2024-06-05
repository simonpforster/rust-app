use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
    pub server: ServerConfig,
    pub notion_client: NotionClientConfig,
}

impl PartialEq for ApplicationConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.logging == other.logging)
            & (self.server == other.server)
            & (self.notion_client == other.notion_client)
    }
}

impl fmt::Display for ApplicationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n", self.logging, self.server, self.notion_client)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    pub log_level: String,
    pub pattern: String,
}

impl PartialEq for LoggerConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.log_level == other.log_level)
            & (self.pattern == other.pattern)
    }
}

impl fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "logging:\n  log-level: {} \n  pattern: {}\n", self.log_level, self.pattern)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub port: u16,
}

impl PartialEq for ServerConfig {
    fn eq(&self, other: &Self) -> bool { self.port == other.port }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "server:\n  port: {}\n", self.port)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NotionClientConfig {
    pub url: String,
    pub path: String,
    pub database_id: String,
    pub notion_version: String,
    pub api_key: String,
}

impl PartialEq for NotionClientConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.url == other.url)
            & (self.path == other.path)
            & (self.database_id == other.database_id)
            & (self.notion_version == other.notion_version)
            & (self.api_key == other.api_key)
    }
}

impl fmt::Display for NotionClientConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "notion_client:\n  url: {}\n  path: {}\n  database_id: {}\n  notion_version: {}\n",
               self.url, self.path, self.database_id, self.notion_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::application_config::{DownstreamOneConfig, LoggerConfig, ServerConfig};

    #[test]
    fn test_app_conf_struct() { // TODO needs refining of what needs to be tested
        let test_config_str: &str = "
            logging:
              log-level: info
              pattern: \"{d} {l} - {m}{n}\"

            server:
              port: 8080

            downstream-one:
              url: \"localhost:8081\"

            notion-client:
              url: \"www.notion.com/\"
              path: \"path/to\"
              database-id: \"1234\"
              notion-version: \"v1\"
              api-key: \"a key\"
            ";
        let parsed_config: ApplicationConfig = serde_yaml::from_str(test_config_str).unwrap();

        let test_against = ApplicationConfig {
            logging: LoggerConfig {
                log_level: "info".to_string(),
                pattern: "{d} {l} - {m}{n}".to_string(),
            },
            server: ServerConfig { port: 8080 },
            downstream_one: DownstreamOneConfig { url: "localhost:8081".to_string() },
            notion_client: NotionClientConfig {
                url: "www.notion.com/".to_string(),
                path: "path/to".to_string(),
                database_id: "1234".to_string(),
                api_key: "a key".to_string(),
                notion_version: "v1".to_string(),
            },
        };

        assert_eq!(parsed_config, test_against)
    }
}
