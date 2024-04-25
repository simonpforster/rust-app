use std::fs::File;
use std::io::BufReader;
use serde::de;
use crate::config::application_config::ApplicationConfig;
use std;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    Message(String),
    Eof,
    Syntax,
    ExpectedString,
    TrailingCharacters,
}

pub fn read() -> ApplicationConfig {

    let file = File::open("resources/config.yaml").unwrap();
    let reader  = BufReader::new(file);
    serde_yaml::from_reader(reader).unwrap()
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