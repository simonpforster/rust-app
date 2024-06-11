use crate::router;
use crate::services::healthcheck_service::HealthcheckService;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::{error, info};
use tokio::net::TcpListener;
use crate::services::notion_service::{NotionDBService};

pub async fn run(
    tcp_listener: TcpListener,
    notion_dbservice: &'static NotionDBService,
    healthcheck_service: &'static HealthcheckService,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Starting server at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(
                    io,
                    service_fn(|r| router::request_handler(r, notion_dbservice, healthcheck_service)),
                )
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}
