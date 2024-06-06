use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response};
use tracing::Instrument;

use crate::routes;
use crate::services::healthcheck_service::HealthcheckService;

pub(crate) type ResponseResult = Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

#[tracing::instrument(name = "request_router")]
pub(crate) async fn request_handler(
    req: Request<hyper::body::Incoming>,
    healthcheck_service: &'static HealthcheckService,
) -> ResponseResult {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/private/status") => routes::private::status(),
        (&Method::GET, "/private/healthcheck") => {
            routes::private::healthcheck(healthcheck_service).in_current_span().await
        }
        (method @ _, path @ _) => routes::utils::no_endpoint(method, path),
    }
}
