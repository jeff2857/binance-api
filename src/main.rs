use colored::*;
use binance_sdk_rs::client::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    if let Err(e) = &client {
        println!("{}{}", "[E]".red(), e);
        return Ok(());
    }

    let client = client.unwrap();
    
    let resp = client.get("/api/v3/ping").await?;

    println!("resp: {}", resp.status());

    Ok(())
}
