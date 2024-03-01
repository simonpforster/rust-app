use service::startup;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use log4rs::init_file;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    init_file("../resources/log4rs.yml", Default::default()).unwrap();

    let server_name = env::var("SERVER_NAME").map_or("R2D2".to_string(), |s| s.to_string());

    let address1: SocketAddr = ([0, 0, 0, 0], 8080).into();
    let listener = TcpListener::bind(address1).await?;
    startup::run(listener, server_name)
        .await
        .expect("Unable to start the server");

    info!("Listening on http://{}", address1);

    Ok(())
}
