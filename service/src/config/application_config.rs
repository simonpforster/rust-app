use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    pub logging: LoggerConfig,
}

#[derive(Deserialize, Debug)]
pub struct LoggerConfig {
    pub log_level: String,
    pub pattern: String,
}