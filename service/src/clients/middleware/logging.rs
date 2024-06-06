use hyper::http::Extensions;
use log::info;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};

pub(in crate::clients) struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        info!("Request: {:?}", req);
        let res = next.run(req, extensions).await;
        info!("Result: {:?}", res);
        res
    }
}