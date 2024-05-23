use crate::router;
use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::{Method, Response, StatusCode};
use log::warn;

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
pub(in crate::routes) fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}
pub(in crate::routes) fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

pub fn no_endpoint(method: &Method, path: &str) -> router::ResponseResult {
    warn!("Endpoint not found {} {}", method.as_str(), path);
    let mut not_found = Response::new(empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}
