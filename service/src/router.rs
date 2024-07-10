use bytes::Bytes;
use http_body_util::combinators::BoxBody;
use hyper::{Method, Request, Response};
use tracing::Instrument;

use crate::routes;
use crate::services::healthcheck_service::HealthcheckService;
use crate::services::notion_service::NotionDBService;

pub type ResponseResult = Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>;

#[tracing::instrument(name = "request_router")]
pub async fn request_handler<'a>(
    req: Request<hyper::body::Incoming>,
    notion_dbservice: &NotionDBService<'a>,
    healthcheck_service: &HealthcheckService<'a>,
) -> ResponseResult {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/private/status") => routes::private::status(),
        (&Method::GET, "/private/healthcheck") => {
            routes::private::healthcheck(healthcheck_service).in_current_span().await
        },
        (&Method::GET, "/v1/tasks") => {
            routes::v1::tasks::tasks(notion_dbservice).in_current_span().await
        },
        (method @ _, path @ _) => routes::utils::no_endpoint(method, path),
    }
}
