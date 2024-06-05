use std::time::Duration;

use async_trait::async_trait;
use hyper::header::AUTHORIZATION;
use reqwest::{Client, header};
use reqwest::header::HeaderMap;

use crate::clients::{DependencyStatus, Healthcheck};

pub fn notion_http_client(api_key: &str, notion_version: &str) -> reqwest::Result<Client> {

    let mut headers = HeaderMap::new();

    let auth_header = header::HeaderValue::from_str(api_key).unwrap();

    let NOTION_VERSION: &'static str = "notion-version";
    let notion_version_header_name = header::HeaderName::from_static(NOTION_VERSION);
    let notion_version_header_value = header::HeaderValue::from_str(notion_version).unwrap();

    headers.insert(AUTHORIZATION, auth_header);
    headers.insert(notion_version_header_name, notion_version_header_value);

    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(5))
        .build()
}

pub struct NotionDatabaseClient {
    pub name: String,
    pub url: String,
    pub database_id: String,
    pub http_client: Client,
}

#[async_trait]
impl Healthcheck for NotionDatabaseClient {
    fn get_name(&self) -> &str { &self.name }

    async fn healthcheck(&self) -> crate::clients::Result<DependencyStatus> {
        let a = self.http_client
            .get(format!("{}{}", &self.url, &self.database_id))
            .send().await?;

        if a.status() == 200 { Ok(DependencyStatus::Healthy) } else {
            Ok(DependencyStatus::Unhealthy(format!("{}: {}", a.status(), a.text().await.unwrap())))
        }
    }
}