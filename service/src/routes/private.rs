use crate::router::ResponseResult;
use crate::routes::utils;
use crate::services::healthcheck_service::HealthcheckService;
use hyper::Response;
use log::info;
use tracing::instrument;
use crate::clients::DependencyStatus;

pub fn status() -> ResponseResult {
    info!("Status polled");
    Ok(Response::new(utils::full("OK")))
}

#[instrument(name = "healthcheck_route")]
pub async fn healthcheck(healthcheck_service: &'static HealthcheckService) -> ResponseResult {
    info!("Healthcheck polled");

    let result = healthcheck_service.check_all().await;
    
    match result {
        Ok(healthcheck_result) => {
            let some: bool = healthcheck_result.iter().fold(true, 
            |a, (_, status)| {
                    match status {
                        DependencyStatus::Unhealthy(_) => false,
                        DependencyStatus::Healthy => a,
                    }
                }
            );
            
            let code = match some {
                true => 200,
                false => 500,
            };
            
            let json = serde_json::to_string_pretty(&healthcheck_result).unwrap();
            let res = Response::builder()
                .status(code)
                .body(utils::full(json))
                .unwrap();
            Ok(res)
        }
        _ => panic!("oops"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn status_ok() {
        let response = status().unwrap_or_default();
        let (parts, some) = response.into_parts();

        let a = String::from_utf8(some.collect().await.unwrap().to_bytes().to_vec()).unwrap();

        assert_eq!(parts.status, 200);
        assert_eq!(a, "OK");
    }
}
