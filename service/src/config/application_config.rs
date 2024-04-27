use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
    pub downstream_one: DownstreamOneConfig,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct LoggerConfig {
    pub log_level: String,
    pub pattern: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct DownstreamOneConfig {
    pub url: String,
}