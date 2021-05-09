use meme_rs::server;
use std::error::Error;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    server::start().await;
    Ok(())
}
