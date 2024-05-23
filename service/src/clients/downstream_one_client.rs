use crate::clients::healthcheck::{Healthcheck, Result, Status};
use async_trait::async_trait;
use bytes::Bytes;
use http_body_util::Empty;
use hyper::{Request, StatusCode, Uri};
use hyper_util::rt::TokioIo;
use log::info;
use std::time::Duration;
use tokio::net::TcpStream;

#[derive(Clone)]
pub struct DownstreamOneClient {
    pub url: Uri,
}

#[async_trait]
impl Healthcheck for DownstreamOneClient {
    async fn healthcheck(&self) -> Result<Status> {
        let host = &self.url.host().expect("uri has no host");
        let port = &self.url.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);

        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                println!("Connection failed: {:?}", err);
            }
        });

        let authority = &self.url.authority().unwrap().clone();

        // Fetch the url...
        let req = Request::builder()
            .uri(&self.url)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;

        match tokio::time::timeout(Duration::from_millis(2000), sender.send_request(req)).await {
            Ok(result) => match result {
                Ok(response) => match response.status() {
                    StatusCode::OK => {
                        info!("downstream 1 healthy");
                        Ok(Status::Healthy)
                    }
                    _ => {
                        info!("downstream 1 unhealthy");
                        Ok(Status::Unhealthy)
                    }
                },
                Err(e) => Err(Box::from(format!("Network error: {:?}", e))),
            },
            Err(_) => Err(Box::from("Timeout: no response in 2 seconds.")),
        }
    }
}
