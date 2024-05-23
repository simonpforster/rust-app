use config::{Case, Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use thiserror::Error;

pub trait Configuration: Sized {
    /// Load this configuration.
    fn load(module: &str) -> Result<Self, Error>;
}

impl<'de, T> Configuration for T
where
    T: Deserialize<'de>,
{
    fn load(module: &str) -> Result<Self, Error> {
        let config_dir = &format!("{}/resources", module);

        let env = env::var("ENVIRONMENT").unwrap_or("local".into());

        Config::builder()
            .add_source(File::with_name(&format!("{}/config.yaml", config_dir)))
            .add_source(File::with_name(&format!(
                "{}/config_{}.yaml",
                config_dir, env
            )))
            .add_source(Environment::default().convert_case(Case::Kebab))
            .build()
            .map_err(Error::Load)?
            .try_deserialize()
            .map_err(Error::Deserialize)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    /// Cannot load the configuration, e.g. because file not found.
    #[error("cannot load configuration")]
    Load(#[source] ConfigError),

    /// Cannot deserialzie the configuration, e.g. because fields are missing.
    #[error("cannot deserialize configuration")]
    Deserialize(#[source] ConfigError),
}
