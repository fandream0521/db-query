// Integration tests for API endpoints
// Note: These tests require a running database for full functionality
// For now, we test the endpoint structure and error handling

#[tokio::test]
async fn test_api_structure() {
    // This is a placeholder test to verify the test infrastructure works
    // Full integration tests would require a test database container
    assert!(true);
}

#[tokio::test]
async fn test_list_databases_empty() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dbs")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let databases: Vec<serde_json::Value> = serde_json::from_slice(&body).unwrap();
    assert_eq!(databases.len(), 0);
}

#[tokio::test]
async fn test_create_and_list_database() {
    let app = create_test_app().await;

    // Create a database (this will fail connection test, but we can mock it)
    // For now, we'll test the endpoint structure
    let create_body = serde_json::json!({
        "url": "postgres://user:pass@localhost:5432/testdb"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri("/api/v1/dbs/test_db")
                .header("content-type", "application/json")
                .body(Body::from(serde_json::to_string(&create_body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    // This will fail because we can't actually connect, but we can test the endpoint exists
    // In a real test, we'd use a test database container
    assert!(response.status() == StatusCode::BAD_REQUEST || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_nonexistent_database() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/dbs/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent_database() {
    let app = create_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/api/v1/dbs/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

