use http_body_util::combinators::BoxBody;
use hyper::Response;
use log::info;
use crate::routes::utils;

pub fn status() -> Result<Response<BoxBody<bytes::Bytes, hyper::Error>>, hyper::Error> {
    info!("Healthcheck polled");
    Ok(Response::new(utils::full("OK")))
}

#[cfg(test)]
mod tests {
    use http_body_util::BodyExt;
    use super::*;

    #[tokio::test]
    async fn test_status() {
        let response = status().unwrap_or_default();
        let (parts, some) = response.into_parts();

        let a = String::from_utf8(
            some.collect().await.unwrap().to_bytes().to_vec()
        ).unwrap();

        assert_eq!(parts.status, 200);
        assert_eq!(a, "OK");
    }
}