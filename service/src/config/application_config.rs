use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
    pub server: ServerConfig,
    pub notion: NotionConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    pub log_level: String,
    pub pattern: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NotionConfig {
    pub client: NotionClientConfig,
    pub db: NotionDBServiceConfig,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NotionClientConfig {
    pub url: String,
    pub notion_version: String,
    pub key: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct NotionDBServiceConfig {
    pub path: String,
    pub id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct MonitoringConfig {
    pub exporter: OtlpExporterConfig,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct OtlpExporterConfig {
    pub url: String,
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

            monitoring:
              otlp-exporter:
                url: \"tempo\"
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
            monitoring: MonitoringConfig {
                exporter: OtlpExporterConfig {
                    url: "tempo".to_string()
                },
            },
        };

        assert_eq!(parsed_config, test_against)
    }
}
