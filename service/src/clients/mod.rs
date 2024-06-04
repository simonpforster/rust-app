pub mod client;
pub mod downstream_one_client;
pub mod notion_client;

use async_trait::async_trait;
use serde::Serialize;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait Healthcheck: Send + Sync {
    fn get_name(&self) -> &str;
    async fn healthcheck(&self) -> Result<DependencyStatus>;
}

#[derive(Serialize)]
pub enum DependencyStatus {
    Healthy,
    Unhealthy(String),
}
