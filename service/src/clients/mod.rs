use std::fmt::Debug;
use async_trait::async_trait;
use serde::{Serialize, Serializer};

mod middleware;
pub mod notion;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait Healthcheck: Send + Sync + Debug {
    fn get_name(&self) -> &str;
    async fn healthcheck(&self) -> Result<DependencyStatus>;
}

#[derive(Clone)]
pub enum DependencyStatus {
    Healthy,
    Unhealthy(String),
}

impl Serialize for DependencyStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match &self {
            DependencyStatus::Healthy => "OK",
            DependencyStatus::Unhealthy(e) =>  e
        })
    }
}
