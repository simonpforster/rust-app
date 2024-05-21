use hyper::{http, Response};
use log::info;
use crate::router::ResponseResult;
use crate::routes::utils;
use crate::clients::downstream_one_client::DownstreamOneClient;

use crate::clients::healthcheck::{Healthcheck, Status};

pub fn status() -> ResponseResult {
    info!("Status polled");
    Ok(Response::new(utils::full("OK")))
}

pub async fn healthcheck() -> ResponseResult {
    info!("Healthcheck polled");

    let url =  http::Uri::from_static("http://localhost:8081/private/status");
    
    let a: DownstreamOneClient = DownstreamOneClient { url };
    
    let result = a.healthcheck().await.unwrap();

    match result {
        Status::Healthy => {
            let res = Response::builder()
                .status(200)
                .body(utils::full("text good")).unwrap();
            Ok(res)
        },
        Status::Unhealthy => {
            let res = Response::builder()
                .status(503)
                .body(utils::full("text bad")).unwrap();
            Ok(res)
        }
    }
    
    
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use super::*;

    #[tokio::test]
    async fn status_ok() {
        let response = status().unwrap_or_default();
        let (parts, some) = response.into_parts();

        let a = String::from_utf8(
            some.collect().await.unwrap().to_bytes().to_vec()
        ).unwrap();

        assert_eq!(parts.status, 200);
        assert_eq!(a, "OK");
    }
}