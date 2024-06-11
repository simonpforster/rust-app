use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
    pub server: ServerConfig,
    pub notion: NotionConfig,
}

impl PartialEq for ApplicationConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.logging == other.logging)
            & (self.server == other.server)
            & (self.notion == other.notion)
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub port: u16,
}

impl PartialEq for ServerConfig {
    fn eq(&self, other: &Self) -> bool { self.port == other.port }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NotionConfig {
    pub client: NotionClientConfig,
    pub db: NotionDBServiceConfig,
}

impl PartialEq for NotionConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.client == other.client)
            & (self.db == other.db)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NotionClientConfig {
    pub url: String,
    pub notion_version: String,
    pub key: String,
}

impl PartialEq for NotionClientConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.url == other.url)
            & (self.notion_version == other.notion_version)
            & (self.key == other.key)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct NotionDBServiceConfig {
    pub path: String,
    pub id: String,
}

impl PartialEq for NotionDBServiceConfig {
    fn eq(&self, other: &Self) -> bool {
        (self.path == other.path)
            & (self.id == other.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::application_config::{LoggerConfig, ServerConfig};

    use super::*;

    #[test]
    fn test_app_conf_struct() { // TODO needs refining of what needs to be tested
        let test_config_str: &str = "
            logging:
              log-level: info
              pattern: \"{d} {l} - {m}{n}\"

            server:
              port: 8080

            notion:
              client:
                url: \"www.notion.com/\"
                notion-version: \"v1\"
                key: \"a key\"
              db:
                path: \"path/to\"
                id: \"1234\"
            ";
        let parsed_config: ApplicationConfig = serde_yaml::from_str(test_config_str).unwrap();

        let test_against = ApplicationConfig {
            logging: LoggerConfig {
                log_level: "info".to_string(),
                pattern: "{d} {l} - {m}{n}".to_string(),
            },
            server: ServerConfig { port: 8080 },
            notion: NotionConfig {
                client: NotionClientConfig {
                    url: "www.notion.com/".to_string(),
                    key: "a key".to_string(),
                    notion_version: "v1".to_string(),
                },
                db: NotionDBServiceConfig {
                    path: "path/to".to_string(),
                    id: "1234".to_string(),
                },
            },
        };

        assert_eq!(parsed_config, test_against)
    }
}
