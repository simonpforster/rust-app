use configloader::Configuration;
use log::{error, info, LevelFilter, SetLoggerError};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::{Config, Handle};
use service::clients::Healthcheck;
use service::config::application_config::{ApplicationConfig, LoggerConfig, OtlpExporterConfig};
use service::services::healthcheck_service::HealthcheckService;
use service::router;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::pin::{pin, Pin};
use std::str::FromStr;
use std::time::Duration;
use handlebars::Handlebars;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use lazy_static::lazy_static;
use opentelemetry::{KeyValue};
use opentelemetry::trace::TraceError;
use tokio::net::TcpListener;
use service::clients::notion::{notion_client, NotionClient};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{Resource, trace};
use opentelemetry_sdk::trace::{RandomIdGenerator, Sampler, Tracer};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::Registry;
use service::clients::notion::notion_db_client::{notion_db_client, NotionDBClient};
use service::services::notion_service::{notion_db_service, NotionDBService};

lazy_static! {
    //load config
    static ref CONFIG: ApplicationConfig = ApplicationConfig::load(&module_path!().to_string(), "RA").unwrap();
    
    //define clients
    static ref NOTION_CLIENT: NotionClient = notion_client(
        &CONFIG.notion.client,
    ).unwrap();
    
    static ref NOTION_DB_CLIENT: NotionDBClient = notion_db_client(
        "notion_db_client".to_string(),
        &NOTION_CLIENT,
        &CONFIG.notion.db
    );

    static ref HANDLEBARS: Handlebars<'static> = Handlebars::new();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //init logger
    logger_setup(&CONFIG.logging)?;

    //init tracing
    let tracer = tracing_setup(&CONFIG.monitoring.exporter)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let subscriber: Layered<OpenTelemetryLayer<Registry, Tracer>, Registry, Registry> = Registry::default().with(telemetry);

    tracing::subscriber::set_global_default(subscriber)?;

    // init clients
    let notion_db_client_ref: &NotionDBClient = &NOTION_DB_CLIENT;

    let vec: Vec<Box<&dyn Healthcheck>> = vec![
        Box::new(notion_db_client_ref),
    ];

    // init healthcheck service
    let healthcheck_service: HealthcheckService = HealthcheckService { clients: vec };

    // init handlebars templates
    let mut handlebars = Handlebars::new();
    
    handlebars.register_template_file("tasks", "./service/resources/templates/tasks.hbs")?;
    
    // init services
    let notion_dbservice: NotionDBService = notion_db_service(&NOTION_DB_CLIENT, handlebars);

    // init http server
    let address1: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), CONFIG.server.port);

    let tcp_listener = TcpListener::bind(address1).await?;

    info!("Server is listening on http://{}", address1);


    info!(
        "Starting server at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );
    
    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        
        let notion = notion_dbservice.clone();
        let hc = healthcheck_service.clone();
        
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(|request| router::request_handler(request, &notion, &hc)),
                )
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}

fn logger_setup(logger_config: &LoggerConfig) -> Result<Handle, SetLoggerError> {
    let level: LevelFilter = LevelFilter::from_str(&logger_config.log_level).unwrap();

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(PatternEncoder::new(&logger_config.pattern)))
        .build();

    let log_conf = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("app::backend::db", level))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();

    log4rs::init_config(log_conf)
}

fn tracing_setup(otlp_exporter_config: &OtlpExporterConfig) -> Result<Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(&otlp_exporter_config.url)
            .with_timeout(Duration::from_secs(3)))
        .with_trace_config(trace::config()
            .with_sampler(Sampler::AlwaysOn)
            .with_id_generator(RandomIdGenerator::default())
            .with_max_events_per_span(64)
            .with_max_attributes_per_span(16)
            .with_max_events_per_span(16)
            .with_resource(Resource::new(vec![KeyValue::new("service.name", "rust-app-service")])))
        .install_batch(opentelemetry_sdk::runtime::Tokio)
}