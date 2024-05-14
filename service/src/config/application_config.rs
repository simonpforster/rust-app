use std::fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
    pub downstream_one: DownstreamOneConfig,
    pub server: ServerConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    pub log_level: String,
    pub pattern: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DownstreamOneConfig {
    pub url: String,
}

impl PartialEq for ApplicationConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.logging == other.logging)
            & (self.server == other.server)
            & (self.downstream_one == other.downstream_one)
    }
}

impl fmt::Display for ApplicationConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}\n{}\n", self.logging, self.server, self.downstream_one)
    }
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

impl PartialEq for ServerConfig {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "server:\n  port: {}\n", self.port)
    }
}

impl PartialEq for DownstreamOneConfig {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl fmt::Display for DownstreamOneConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "downstream_one:\n  url: {}\n", self.url)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::application_config::{DownstreamOneConfig, LoggerConfig, ServerConfig};
    use super::*;

    #[test]
    fn test_app_conf_struct() { // TODO needs refining of what needs to be tested
        let test_config_str: &str =
            "
            logging:
              log-level: info
              pattern: \"{d} {l} - {m}{n}\"

            server:
              port: 8080

            downstream-one:
              url: \"localhost:8081\"\
            ";
        let parsed_config: ApplicationConfig = serde_yaml::from_str(test_config_str).unwrap();

        let test_against = ApplicationConfig {
            logging: LoggerConfig {
                log_level: "info".to_string(),
                pattern: "{d} {l} - {m}{n}".to_string(),
            },
            downstream_one: DownstreamOneConfig { url: "localhost:8081".to_string() },
            server: ServerConfig { port: 8080 },
        };

        assert_eq!(parsed_config, test_against)
    }
}


