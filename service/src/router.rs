use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response};
use crate::routes;

pub(crate) type ResponseResult = Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

pub(crate) async fn request_handler(
    req: Request<hyper::body::Incoming>,
    name: &str,
) -> ResponseResult {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/private/status") => {
            routes::private::status()
        },
        (method @ _, path @ _) => {
            routes::utils::no_endpoint(method, path)
        }
    }
}