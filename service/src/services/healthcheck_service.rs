use std::collections::HashMap;
use crate::clients::{Healthcheck, Result, DependencyStatus};
use log::error;
use tokio::task::JoinSet;
use tracing::{instrument, Instrument};

#[derive(Debug)]
pub struct HealthcheckService {
    pub clients: Vec<Box<&'static dyn Healthcheck>>,
}

impl HealthcheckService {
    
    #[instrument(name = "healthcheck_all")]
    pub async fn check_all(&'static self) -> Result<HashMap<String, DependencyStatus>> {
        
        let mut set: JoinSet<(String, Result<DependencyStatus>)> = JoinSet::new();
        
        for client in &self.clients {
            let _ = set.spawn(async move {
                (String::from(client.get_name()), client.healthcheck().await)
            }.in_current_span());
        }

        let mut data: HashMap<String, DependencyStatus> = HashMap::new();
        
        while let Some(res) = set.join_next().await {
            match res {
                Ok((name, result)) => match result {
                    Ok(d) => { data.insert(name, d); }
                    Err(e) => { data.insert(name, DependencyStatus::Unhealthy(e.to_string())); }
                },
                Err(e) => error!("JoinError: {}", e),
            }
        }
        Ok(data)
    }
}
