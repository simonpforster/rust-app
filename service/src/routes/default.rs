use hyper::Response;
use log::info;
use crate::router::ResponseResult;
use crate::routes::utils::full;

pub(crate) fn route(name: &str) -> ResponseResult {
    info!("Hello reached");
    Ok(Response::new(full(format!("Hello from server {}", name))))
}