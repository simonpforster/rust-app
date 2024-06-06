use async_trait::async_trait;
use hyper::Method;
use tracing::instrument;
use crate::clients::{DependencyStatus, Healthcheck};
use crate::clients::notion::NotionClient;
use crate::config::application_config::NotionDBServiceConfig;

pub fn notion_db_client(name: String, notion_client: &'static NotionClient, notion_db_config: &'static NotionDBServiceConfig) -> NotionDBClient {
    NotionDBClient {
        notion_client,
        name: name.to_string(),
        database_id: notion_db_config.database_id.to_string(),
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

#[async_trait]
impl Healthcheck for NotionDBClient {
    fn get_name(&self) -> &str { &self.name }

    #[instrument(name = "notion-database-client")]
    async fn healthcheck(&self) -> crate::clients::Result<DependencyStatus> {
        let a = self.notion_client.request(
            Method::GET,
            format!("{}{}", &self.path,  &self.database_id)
        ).send().await?;

        if a.status() == 200 { Ok(DependencyStatus::Healthy) } else {
            Ok(DependencyStatus::Unhealthy(format!("{}: {}", a.status(), a.text().await.unwrap())))
        }
    }
}