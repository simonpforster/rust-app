use std::collections::HashMap;

use futures_util::future::join_all;
use tracing::{instrument, Instrument};

use crate::clients::{DependencyStatus, Healthcheck, Result};

#[derive(Clone, Debug)]
pub struct HealthcheckService<'serv> {
    pub clients: Vec<Box<&'serv dyn Healthcheck>>,
}

impl<'serv> HealthcheckService<'serv> {
    #[instrument(name = "healthcheck_all")]
    pub async fn check_all<'check>(&'serv self) -> Result<HashMap<String, DependencyStatus>> {

        let mut v = Vec::new();

        for client in &self.clients {
            v.push(async move { (String::from(client.get_name()), client.healthcheck().await) }.in_current_span());
        }

        let client_results: Vec<(String, Result<DependencyStatus>)> = join_all(v).await;
        
        let mut data: HashMap<String, DependencyStatus> = HashMap::new();
        
        client_results.iter().for_each(|(name, status)| {
            match status {
                Ok(s) => data.insert(name.to_owned(), s.clone()),
                Err(e) => data.insert(name.to_owned(), DependencyStatus::Unhealthy(e.to_string())),
            };
        });
        
        Ok(data)
    }
}
