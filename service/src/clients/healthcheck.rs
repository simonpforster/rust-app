use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait Healthcheck: Send + Sync {
    fn get_name(&self) -> &str;
    async fn healthcheck(&self) -> Result<Status>;
}

pub enum Status {
    Healthy,
    Unhealthy(String),
}
