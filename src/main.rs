use std::net::TcpListener;

use mailbridge::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let server = run(listener).await?;
    server.await
}
