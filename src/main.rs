#![allow(unused)]

use std::env;

use log::{error, info};
use binance_sdk_rs::{client::client::Client, market::market, wallet::wallet};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env::set_var("RUST_LOG", "info");

    // test APIKEY and SECRETKEY
    //env::set_var("APIKEY", "your api key");
    //env::set_var("SECRETKEY", "you secret key");

    let client = match Client::with_proxy("http://172.21.48.1:7890".into()) {
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
    //let exchange_info_sol = market::exchange_info_symbol(&client, &symbol).await?;
    //println!("resp exchange_info_sol: {:?}", exchange_info_sol);

    //let depth = market::depth(&client, &symbol, 2).await?;
    //println!("resp depth: {:?}", depth);

    let resp_capital_all = wallet::capital_all(&client).await?;
    println!("resp_capital_all: {:?}", resp_capital_all);

    Ok(())
}
