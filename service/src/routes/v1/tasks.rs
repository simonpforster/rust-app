use std::pin::Pin;
use handlebars::Handlebars;
use hyper::Response;
use tracing::{Instrument, instrument};
use crate::router::ResponseResult;
use crate::routes::utils;
use crate::services::notion_service::NotionDBService;

#[instrument(name = "tasks_route")]
pub async fn tasks<'a>(notion_dbservice: &NotionDBService<'_>, handlebars: &Handlebars<'a>) -> ResponseResult {
    let tasks = notion_dbservice.get_entries().in_current_span().await.unwrap();

    let body: String = handlebars.render("tasks", &tasks).unwrap();
    
    let res = Response::builder()
        .status(200)
        .body(utils::full(body))
        .unwrap();
    Ok(res)
}