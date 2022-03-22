use std::env;

use log::{error, info};
use binance_sdk_rs::{client::client::Client, market::market};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "info");

    // test APIKEY and SECRETKEY
    env::set_var("APIKEY", "MY_APIKEY");
    env::set_var("SECRETKEY", "MY_SECRETKEY");

    let client = match Client::with_proxy("http://127.0.0.1:7890".into()) {
        Ok(client) => client,
        Err(err) => {
            error!("{}", err);
            return Ok(());
        }
    };
    
    info!("market ping");
    let ping_body = market::ping(&client).await?;
    println!("resp: {:?}", ping_body);

    let symbol = String::from("SOLUSDT");
    let exchange_info_sol = market::exchange_info_symbol(&client, &symbol).await?;
    println!("resp exchange_info_sol: {:?}", exchange_info_sol);

    Ok(())
}
