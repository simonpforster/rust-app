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