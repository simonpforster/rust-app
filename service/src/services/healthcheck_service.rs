use std::collections::HashMap;
use crate::clients::{Healthcheck, Result, DependencyStatus};
use log::error;
use tokio::task::JoinSet;

#[derive(Clone)]
pub struct HealthcheckService {
    pub clients: Vec<Box<&'static dyn Healthcheck>>,
}

impl HealthcheckService {
    pub async fn check_all(&'static self) -> Result<HashMap<String, DependencyStatus>> {
        let mut set: JoinSet<(String, Result<DependencyStatus>)> = JoinSet::new();

        for client in &self.clients {
            set.spawn(async move { 
                (String::from(client.get_name()), client.healthcheck().await)
             });
        }

        let mut data: HashMap<String, DependencyStatus> = HashMap::new();
        
        while let Some(res) = set.join_next().await {
            match res {
                Ok((name, Ok(DependencyStatus::Healthy))) => {
                    data.insert(name, DependencyStatus::Healthy);
                }
                Ok((name, Ok(DependencyStatus::Unhealthy(e)))) => {
                    data.insert(name, DependencyStatus::Unhealthy(e));
                }
                Ok((name, Err(e))) => {
                    error!("Some Error: {}", e);
                    data.insert(name, DependencyStatus::Unhealthy(e.to_string()));
                }
                Err(e) => { // TODO I dont like the way we handle this
                    error!("JoinError: {}", e);
                }
            }
        }

        Ok(data)
    }
}
