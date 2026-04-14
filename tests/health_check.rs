use mailbridge::run;

async fn spawn_app() {
    let srv = run().await.expect("should start");

    let _ = tokio::spawn(srv);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success(), "health check works");
    assert_eq!(response.content_length(), Some(0));
}
