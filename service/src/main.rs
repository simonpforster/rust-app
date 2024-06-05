use configloader::Configuration;
use log::{info, LevelFilter, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use service::clients::Healthcheck;
use service::config::application_config::{ApplicationConfig, LoggerConfig};
use service::services::healthcheck_service::HealthcheckService;
use service::startup;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener;
use service::clients::notion_database_client::{notion_http_client, NotionDatabaseClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // init config
    let config: ApplicationConfig = ApplicationConfig::load(&module_path!().to_string()).unwrap();

    logger_setup(&config.logging).unwrap();

    info!("Loading config: \n{}", config);

    // init clients
    let url: String = format!("{}{}", &config.notion_client.url, &config.notion_client.path);
    let notion_client: NotionDatabaseClient = NotionDatabaseClient {
        name: String::from("notion_database_client_1"),
        url,
        database_id: config.notion_client.database_id,
        http_client: notion_http_client(&config.notion_client.api_key, &config.notion_client.notion_version)?,
    };

    let boxed_notion_client: &NotionDatabaseClient = Box::leak(Box::new(notion_client)) as &'static _;

    let vec: Vec<Box<&dyn Healthcheck>> = vec![
        Box::new(boxed_notion_client),
    ];

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
