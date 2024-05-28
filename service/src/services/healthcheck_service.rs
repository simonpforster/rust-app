use crate::clients::healthcheck::{Healthcheck, Result, Status};
use log::error;
use tokio::task::JoinSet;

#[derive(Clone)]
pub struct HealthcheckService {
    pub clients: Vec<Box<&'static dyn Healthcheck>>,
}

impl HealthcheckService {
    pub async fn check_all(&'static self) -> Result<Vec<(String, String)>> {
        let mut set: JoinSet<(String, Result<Status>)> = JoinSet::new();

        for client in &self.clients {
            set.spawn(async move { 
                (String::from(client.get_name()), client.healthcheck().await)
             });
        }

        let mut data: Vec<(String, String)> = Vec::new();

        while let Some(res) = set.join_next().await {
            match res {
                Ok((name, Ok(Status::Healthy))) => {
                    data.insert(0, (name, String::from("OK")));
                }
                Ok((name, Ok(Status::Unhealthy(e)))) => {
                    data.insert(0, (name, e));
                }
                Err(e) => {
                    error!("JoinError: {}", e);
                }
                Ok((name, Err(e))) => {
                    error!("Some Error: {}", e);
                    data.insert(0, (name, e.to_string()));
                }
            }
        }

        Ok(data)
    }
}
