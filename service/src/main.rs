use service::startup;
use std::net::SocketAddr;
use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use tokio::net::TcpListener;
use log::{info, LevelFilter, logger};
use log4rs::config::{Appender, Logger, Root};
use service::config::application_config::{ApplicationConfig, LoggerConfig};
use service::config::config_reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let config: ApplicationConfig = config_reader::read()?;

    let server_name: String = String::from("SERVER IS NAME");

    let address1: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, server_name)
        .await
        .expect("Unable to start the server");

    info!("Listening on http://{}", address1);

    Ok(())
}

fn logger_setup(logger_config: &mut LoggerConfig) {



    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("app::backend::db", ))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

}