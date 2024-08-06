#[cfg(test)]
mod tests {
    use reqwest::StatusCode;

    #[tokio::test]
    async fn check_v1_tasks() {
        let response = reqwest::get("http://localhost:8080/v1/tasks").await.unwrap();
        let status = response.status();
        let body = response.text().await.unwrap();

        assert_eq!(status, StatusCode::OK);
        // assert_eq!(body, "{\n  \"notion_db_client\": \"OK\"\n}")
    }
}
