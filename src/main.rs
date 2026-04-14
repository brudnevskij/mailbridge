use mailbridge::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = run().await?;
    server.await
}
