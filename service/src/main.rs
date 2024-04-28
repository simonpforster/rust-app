use service::startup;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::{Config, Handle};
use tokio::net::TcpListener;
use log::{info, LevelFilter, SetLoggerError};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use service::config::application_config::{ApplicationConfig, LoggerConfig};
use service::config::config_reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    info!("Loading config.");
    let config: ApplicationConfig = config_reader::load();

    logger_setup(&config.logging).unwrap();

    let server_name: String = String::from("SERVER IS NAME");

    let address1: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.server.port);

    info!("Starting server on port {}.", &config.server.port);
    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, server_name)
        .await
        .expect("Unable to start the server");

    info!("Server is listening on http://{}", address1);

    Ok(())
}

fn logger_setup(logger_config: &LoggerConfig) -> Result<Handle, SetLoggerError> {

    let level: LevelFilter = LevelFilter::from_str(&logger_config.log_level).unwrap();

    let stdout: ConsoleAppender =
        ConsoleAppender::builder()
            .target(Target::Stdout)
            .encoder(Box::new(PatternEncoder::new(&logger_config.pattern)))
            .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(
            Logger::builder().build("app::backend::db",  level)
        )
        .build(
            Root::builder().appender("stdout").build(level)
        )
        .unwrap();

    log4rs::init_config(config)
}