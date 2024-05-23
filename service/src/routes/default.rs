use crate::router::ResponseResult;
use crate::routes::utils::full;
use hyper::Response;
use log::info;

pub(crate) fn route(name: &str) -> ResponseResult {
    info!("Hello reached");
    Ok(Response::new(full(format!("Hello from server {}", name))))
}
