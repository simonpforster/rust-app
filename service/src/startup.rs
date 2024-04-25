use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use log::{error, info};
use tokio::net::TcpListener;
use crate::router;

pub async fn run(
    tcp_listener: TcpListener,
    name: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Starting server at: {}:{}",
        &tcp_listener.local_addr()?.ip().to_string(),
        tcp_listener.local_addr()?.port()
    );

    loop {
        let (stream, _) = tcp_listener.accept().await?;
        let io = TokioIo::new(stream);

        let name = name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(|r| router::request_handler(r, name.as_str())))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }
}



