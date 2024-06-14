use serde::Serialize;
use tracing::instrument;

use crate::clients::notion::notion_db_client::NotionDBClient;
use crate::model::notion_task::{Results};
use crate::model::task::Task;

pub fn notion_db_service(notion_db_client: &'static NotionDBClient) -> NotionDBService {
    NotionDBService { 
        notion_db_client,
    }
}

#[derive(Debug)]
pub struct NotionDBService {
   notion_db_client: &'static NotionDBClient,
}

impl NotionDBService {

    #[instrument]
    pub async fn get_entries(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let filter = r#"
        {
          "filter": {
            "or": [
              {
                "property": "Status",
                "status": {
                  "equals": "Not started"
                }
              },
              {
                "property": "Status",
                "status": {
                  "equals": "Planned"
                }
              }
            ]
          },
          "sorts": [
            {
              "property": "Created",
              "direction": "descending"
            }
          ]
        }"#;

        let res = &self.notion_db_client.query(filter).await?;

        let results: Results = Results::deserialize(res);

        let a = results.results.iter().map(|a| { a.properties.to_task()}).collect::<Vec<Task>>();

        let b= serde_json::to_string(&a)?;
        
        Ok(b)
    }

}