use std;
use std::env;
use std::fmt;
use std::fmt::Display;
use configured::{CONFIG_DIR, Configured};
use log::warn;
use serde::de;
use crate::config::application_config::ApplicationConfig;

#[derive(Debug)]
pub enum Error {
    Message(String),
    Eof,
    Syntax,
    ExpectedString,
    TrailingCharacters,
}

fn default_configured_vars() -> Result<(), Error> {
    drop(env::var(CONFIG_DIR).map_err(|_| {
        warn!("Loading default configuration folder.");
        env::set_var(CONFIG_DIR, "service/resources")
    }));
    Ok(())
}

pub fn load() -> ApplicationConfig {
    drop(default_configured_vars());
    ApplicationConfig::load().unwrap()
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::Syntax => formatter.write_str("wrong syntax"),
            Error::ExpectedString => formatter.write_str("expected a string, did not get"),
            Error::TrailingCharacters => formatter.write_str("trailing characters"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use std::env;
    use configured::CONFIG_DIR;
    use crate::config::config_reader::default_configured_vars;

    #[test]
    fn test_default_variables() {
        env::remove_var(CONFIG_DIR);

        drop(default_configured_vars());

        assert_eq!(env::var(CONFIG_DIR).unwrap(), "service/resources")
    }

    #[test]
    fn test_override_default_variables() {
        env::set_var(CONFIG_DIR, "resources");

        drop(default_configured_vars());

        assert_eq!(env::var(CONFIG_DIR).unwrap(), "resources")
    }

}