pub mod notion_db_client;

use std::time::Duration;

use hyper::header::AUTHORIZATION;
use reqwest::{Client, header, Method};
use reqwest::header::HeaderMap;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, Extension, RequestBuilder};
use reqwest_tracing::OtelName;
use tracing::instrument;

use crate::clients::middleware::logging::LoggingMiddleware;
use crate::clients::middleware::tracing::TracingMiddleware;
use crate::config::application_config::NotionClientConfig;

pub fn notion_client(notion_client_config: &NotionClientConfig) -> reqwest::Result<NotionClient> {
    let client = Client::builder()
        .default_headers(notion_headers(&notion_client_config.api_key, &notion_client_config.notion_version))
        .timeout(Duration::from_secs(5))
        .build()?;

    let wrap_client = ClientBuilder::new(client)
        .with_init(Extension(OtelName("notion_client".into())))
        .with(LoggingMiddleware)
        .with(TracingMiddleware)
        .build();

    Ok(NotionClient {
        url: notion_client_config.url.to_string(),
        http_client: wrap_client,
    })
}

fn notion_headers(api_key: &str, notion_version: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    let auth_header = header::HeaderValue::from_str(api_key).unwrap();

    static NOTION_VERSION: &'static str = "notion-version";
    let notion_version_header_name = header::HeaderName::from_static(NOTION_VERSION);
    let notion_version_header_value = header::HeaderValue::from_str(notion_version).unwrap();

    headers.insert(AUTHORIZATION, auth_header);
    headers.insert(notion_version_header_name, notion_version_header_value);

    headers
}

#[derive(Debug)]
pub struct NotionClient {
    pub url: String,
    pub http_client: ClientWithMiddleware,
}

impl NotionClient {
    #[instrument(name = "notion-client")]
    pub fn request(&self, method: Method, path: String) -> RequestBuilder {
        self.http_client.request(method, format!("{}{}", &self.url, path))
    }
}

