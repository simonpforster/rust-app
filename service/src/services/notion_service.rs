use tracing::instrument;

use crate::clients::notion::notion_db_client::NotionDBClient;
use crate::model::notion_task::{Results};
use crate::model::task::{Task, Tasks};

pub fn notion_db_service(notion_db_client: &NotionDBClient) -> NotionDBService {
    NotionDBService {
        notion_db_client,
    }
}

#[derive(Clone, Debug)]
pub struct NotionDBService<'serv> {
    notion_db_client: &'serv NotionDBClient,
}

impl<'serv> NotionDBService<'serv> {
    #[instrument]
    pub async fn get_entries(&self) -> Result<Tasks, Box<dyn std::error::Error + Send + Sync>> {
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

        let tasks = Tasks { tasks: results.results.iter().map(|a| { a.properties.to_task() }).collect::<Vec<Task>>() };

        Ok(tasks)
    }
}