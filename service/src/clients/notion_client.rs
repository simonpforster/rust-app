use async_trait::async_trait;
use hyper::Uri;
use crate::clients::{Healthcheck, DependencyStatus};

pub struct DownstreamOneClient {
    pub name: String,
    pub url: Uri,
}

#[async_trait]
impl Healthcheck for DownstreamOneClient {
    fn get_name(&self) -> &str { &self.name }

    async fn healthcheck(&self) -> crate::clients::Result<DependencyStatus> {
        todo!()
    }
}