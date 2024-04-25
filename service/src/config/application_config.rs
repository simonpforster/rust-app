use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApplicationConfig {
    logging: LoggerConfig,
}

pub struct LoggerConfig {
    log_level: String,
    pattern: String,
}