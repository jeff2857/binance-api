use std::env;

use log::{error, info};
use binance_sdk_rs::client::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "info");

    // test APIKEY and SECRETKEY
    env::set_var("APIKEY", "MY_APIKEY");
    env::set_var("SECRETKEY", "MY_SECRETKEY");

    let client = Client::new();

    if let Err(e) = &client {
        error!("{}", e);
        return Ok(());
    }

    let client = client.unwrap();

    info!("{:?}", client);
    
    let resp = client.get("/api/v3/ping").await?;

    println!("resp: {}", resp.status());

    Ok(())
}
