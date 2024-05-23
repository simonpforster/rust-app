use crate::clients::healthcheck::{Healthcheck, Result, Status};
use log::{error, info};
use tokio::task::JoinSet;

#[derive(Clone)]
pub struct HealthcheckService {
    pub clients: Vec<Box<&'static dyn Healthcheck>>,
}

impl HealthcheckService {
    pub async fn check_all(&'static self) -> Result<Status> {
        let mut set: JoinSet<Result<Status>> = JoinSet::new();

        for client in &self.clients {
            set.spawn(async move { client.healthcheck().await });
        }

        while let Some(res) = set.join_next().await {
            match res {
                Ok(Ok(Status::Healthy)) => {
                    info!("a downstream healthy");
                }
                Ok(Ok(Status::Unhealthy)) => {
                    info!("a downstream unhealthy")
                }
                Err(e) => {
                    error!("JoinError: {}", e);
                }
                Ok(Err(e)) => {
                    error!("Some Error: {}", e);
                }
            }
        }

        Ok(Status::Healthy)
    }
}
