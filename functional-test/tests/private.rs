#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    #[tokio::test]
    async fn check_healthcheck() {
        let response = reqwest::get("http://localhost:8080/private/healthcheck").await.unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "{\n  \"notion_db_client\": \"OK\"\n}")
    }

    #[tokio::test]
    async fn check_status() {
        let response = reqwest::get("http://localhost:8080/private/status").await.unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "OK")
    }
}
