use std::env;

use log::{error, info};
use binance_sdk_rs::client::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "info");

    // test APIKEY and SECRETKEY
    env::set_var("APIKEY", "MY_APIKEY");
    env::set_var("SECRETKEY", "MY_SECRETKEY");

    let client = Client::with_proxy("http://127.0.0.1:7890".to_string());

    if let Err(e) = &client {
        error!("{}", e);
        return Ok(());
    }

    let client = client.unwrap();

    info!("{:?}", &client);
    
    let resp = client.get("/api/v3/ping").await?;

    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    println!("resp: {:?}", body_bytes);

    Ok(())
}
