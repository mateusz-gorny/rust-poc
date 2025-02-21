#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn test_create_post() {
        let url = "http://127.0.0.1:3000/posts";
        let body = json!({ "title": "Test Post", "content": "This is a test post." });

        let client = reqwest::Client::new();
        let response = client.post(url)
            .json(&body)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::OK);

        let json_response: serde_json::Value = response.json().await.expect("Failed to parse response");
        assert!(json_response.is_object(), "Response should be a JSON object");
        assert_eq!(json_response["title"], "Test Post");
        assert_eq!(json_response["content"], "This is a test post.");
    }

    #[tokio::test]
    async fn test_get_posts() {
        let url = "http://127.0.0.1:3000/posts";
        let client = reqwest::Client::new();
        let response = client.get(url)
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(response.status(), StatusCode::OK);

        let json_response: serde_json::Value = response.json().await.expect("Failed to parse response");
        assert!(json_response.is_array(), "Response should be a JSON array");

        let posts_array = json_response.as_array().expect("Failed to convert JSON to array");
        assert!(!posts_array.is_empty(), "Post array should not be empty");
        assert!(posts_array.len() > 1, "Expected 1 post in the database");

        if let Some(first_post) = posts_array.first() {
            assert!(first_post["title"].is_string(), "Title should be a string");
            assert!(first_post["content"].is_string(), "Content should be a string");
        }
    }
}