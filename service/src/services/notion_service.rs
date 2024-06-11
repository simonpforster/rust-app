use serde::Serialize;
use serde_json::{json, Value};
use tracing::instrument;

use crate::clients::notion::notion_db_client::NotionDBClient;

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

        // let proc: Value = serde_json::from_str(res.as_str())?;
        // 
        // let objectType = &proc["object"];
        // let list = &proc["results"];




        Ok(res.to_string())
    }

}