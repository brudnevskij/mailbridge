use std::net::TcpListener;

use mailbridge::run;

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let srv = run(listener).await.expect("should start");

    let _ = tokio::spawn(srv);

    format!("127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get(format!("http://{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success(), "health check works");
    assert_eq!(response.content_length(), Some(0));
}
