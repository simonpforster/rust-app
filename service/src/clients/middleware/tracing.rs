use hyper::http::Extensions;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use tracing::{event, Instrument, Level, span};

pub(in crate::clients) struct TracingMiddleware;

#[async_trait::async_trait]
impl Middleware for TracingMiddleware {
    
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,    
        next: Next<'_>,
    ) -> Result<Response> {
        event!{Level::INFO, "{:?}", req}
        let res: Result<Response> = next.run(req, extensions).instrument(span!(Level::INFO, "reqwest")).await;
        event!(Level::INFO, "{:?}", res);
        res
    }
}