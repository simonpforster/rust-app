use configloader::Configuration;
use hyper::{http, Uri};
use log::{info, LevelFilter, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use service::clients::downstream_one_client::DownstreamOneClient;
use service::clients::healthcheck::Healthcheck;
use service::config::application_config::{ApplicationConfig, LoggerConfig};
use service::services::healthcheck_service::HealthcheckService;
use service::startup;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // init config
    let config: ApplicationConfig = ApplicationConfig::load(&module_path!().to_string()).unwrap();

    logger_setup(&config.logging).unwrap();

    info!("Loading config: \n{}", config);

    // init clients
    let url: Uri = http::Uri::from_str(&config.downstream_one.url).unwrap();
    let downstream_one_client: DownstreamOneClient = DownstreamOneClient { name: String::from("downstream_one"),  url: url.clone() };
    let downstream_one_client_dupe: DownstreamOneClient = DownstreamOneClient { name: String::from("downstream_one_dupe"),  url: url.clone() };

    let boxed_client: &DownstreamOneClient =
        Box::leak(Box::new(downstream_one_client)) as &'static _;

    let boxed_client_dupe: &DownstreamOneClient =
        Box::leak(Box::new(downstream_one_client_dupe)) as &'static _;


    let vec: Vec<Box<&dyn Healthcheck>> = vec![Box::new(boxed_client), Box::new(boxed_client_dupe)];
    // init healthcheck service

    let healthcheck_service: HealthcheckService = HealthcheckService { clients: vec };

    let boxed_check: &HealthcheckService = Box::leak(Box::new(healthcheck_service)) as &'static _;

    // init http server
    let address1: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.server.port);

    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, boxed_check)
        .await
        .expect("Unable to start the server");

    info!("Server is listening on http://{}", address1);

    Ok(())
}

fn logger_setup(logger_config: &LoggerConfig) -> Result<Handle, SetLoggerError> {
    let level: LevelFilter = LevelFilter::from_str(&logger_config.log_level).unwrap();

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(&logger_config.pattern)))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("app::backend::db", level))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();

    log4rs::init_config(config)
}
