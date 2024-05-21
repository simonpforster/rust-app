
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub trait Healthcheck {
    async fn healthcheck(&self) -> Result<Status>;
}

pub enum Status {
    Healthy,
    Unhealthy
}