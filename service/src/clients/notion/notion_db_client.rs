
use async_trait::async_trait;
use hyper::{header, Method, StatusCode};
use log::error;
use tracing::{instrument, Instrument};

use crate::clients::{DependencyStatus, Healthcheck};
use crate::clients::notion::NotionClient;
use crate::config::application_config::NotionDBServiceConfig;

pub fn notion_db_client(name: String, notion_client: &'static NotionClient, notion_db_config: &'static NotionDBServiceConfig) -> NotionDBClient {
    NotionDBClient {
        notion_client,
        name: name.to_string(),
        database_id: notion_db_config.id.to_string(),
        path: notion_db_config.path.to_string(),
    }
}

#[derive(Debug)]
pub struct NotionDBClient {
    notion_client: &'static NotionClient,
    name: String,
    database_id: String,
    path: String,
}

impl NotionDBClient {

    #[instrument(name = "notion-db-client", fields(filter = filter))]
    pub async fn query(&self, filter: &str) -> crate::clients::Result<String> {
        
        let res = self.notion_client.request(
            Method::POST,
            format!("{}{}/query", &self.path, &self.database_id),
        )
            .header("content-type", "application/json")
            .body(filter.to_string())
            .send().in_current_span().await?;
        

        
        let status = &res.status();
        match status { 
            &StatusCode::OK => Ok(res.text().await?),
            e => {
                error!("Error!!!!: {}", e);
                Ok(res.text().await?)
            },
        }
    }
}

#[async_trait]
impl Healthcheck for NotionDBClient {
    fn get_name(&self) -> &str { &self.name }

    #[instrument(name = "notion-db-client")]
    async fn healthcheck(&self) -> crate::clients::Result<DependencyStatus> {
        let res = self.notion_client.request(
            Method::GET,
            format!("{}{}", &self.path, &self.database_id),
        ).send().await?;

        if res.status() == 200 { Ok(DependencyStatus::Healthy) } else {
            Ok(DependencyStatus::Unhealthy(format!("{}: {}", res.status(), res.text().await.unwrap())))
        }
    }
}