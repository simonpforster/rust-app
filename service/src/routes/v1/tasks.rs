use hyper::Response;
use tracing::{Instrument, instrument};
use crate::router::ResponseResult;
use crate::routes::utils;
use crate::services::notion_service::NotionDBService;

#[instrument(name = "tasks_route")]
pub async fn tasks(notion_dbservice: &NotionDBService) -> ResponseResult {
    let a = notion_dbservice.get_entries().in_current_span().await.unwrap();

    let res = Response::builder()
        .status(200)
        .body(utils::full(a))
        .unwrap();
    Ok(res)
}