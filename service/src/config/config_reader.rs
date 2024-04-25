use std::fs;
use tokio::fs::File;
use tokio::io::BufReader;
use crate::config::application_config::ApplicationConfig;

pub fn read() -> Result<ApplicationConfig, Err> {

    let file = File::open("resources/config.yaml")?;
    let reader  = BufReader::new(file);
    let config: Result<ApplicationConfig, Err> = serde_yaml::from_reader(reader)?;
    config
}