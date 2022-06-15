use hyper::body::Bytes;

use crate::http::client::{Client, RequestParam};


const URL_PING: &str = "/api/v3/ping";
const URL_TIME: &str = "/api/v3/time";
const URL_EXCHANGE_INFO: &str = "/api/v3/exchangeInfo";
const URL_DEPTH: &str = "/api/v3/depth";
const URL_TRADES: &str = "/api/v3/trades";
const URL_TRADES_HISTORY: &str = "/api/v3/historicalTrades";
const URL_TRADES_AGG: &str = "/api/v3/aggTrades";
const URL_KLINES: &str = "/api/v3/klines";
const URL_PRICE_AVG: &str = "/api/v3/avgPrice";
const URL_TICKER_24HR: &str = "/api/v3/ticker/24hr";
const URL_TICKER_PRICE: &str = "/api/v3/ticker/price";
const URL_TICKER_BOOK: &str = "/api/v3/ticker/bookTicker";


pub async fn ping(client: &Client) -> Result<Bytes, String> {
    let resp = client.get(URL_PING).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    Ok(body_bytes)
}

pub async fn time(client: &Client) -> Result<Bytes, String> {
    let resp = client.get(URL_TIME).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    Ok(body_bytes)
}

pub async fn exchange_info(client: &Client) -> Result<Bytes, String> {
    let resp = client.get(URL_EXCHANGE_INFO).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        }
    };
    Ok(body_bytes)
}

pub async fn exchange_info_symbol(client: &Client, symbol: &String) -> Result<Bytes, String> {
    let param = vec![RequestParam{key: String::from("symbol"), value: String::from(symbol)}];
    let resp = client.get_with_param(URL_EXCHANGE_INFO, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn exchange_info_symbols(client: &Client, symbols: &Vec<String>) -> Result<Bytes, String> {
    let mut symbols_str = String::from("[");
    for s in symbols.iter() {
        symbols_str.push_str(s.as_str());
    }
    symbols_str.push_str(&"]");

    let param = vec![RequestParam{key: String::from("symbol"), value: symbols_str}];
    let resp = client.get_with_param(URL_EXCHANGE_INFO, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn depth(client: &Client, symbol: &String, limit: u32) -> Result<Bytes, String> {
    let limit_options: [u32; 8] = [5, 10, 20, 50, 100, 500, 1000, 5000];
    if let None = limit_options.iter().find(|&&x| x == limit) {
        return Err(format!("limit must be one of {:?}", &limit_options));
    }

    let param = vec![
        RequestParam{key: String::from("symbol"), value: String::from(symbol)},
        RequestParam{key: String::from("limit"), value: limit.to_string()},
    ];
    let resp = client.get_with_param(URL_DEPTH, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn trades(client: &Client, symbol: &String, limit: u32) -> Result<Bytes, String> {
    if limit > 1000 {
        return Err("limit must be less than or equal to 1000".to_string());
    }

    let param = vec![
        RequestParam{key: String::from("symbol"), value: String::from(symbol)},
        RequestParam{key: String::from("limit"), value: limit.to_string()},
    ];
    let resp = client.get_with_param(URL_TRADES, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn historical_trades(client: &Client, symbol: &String, limit: u32, from_id: Option<u64>) -> Result<Bytes, String> {
    if limit > 1000 {
        return Err("limit must be less than or equal to 1000".to_string());
    }

    let mut param = vec![
        RequestParam{key: String::from("symbol"), value: String::from(symbol)},
        RequestParam{key: String::from("limit"), value: limit.to_string()},
    ];
    if let Some(from_id) = from_id {
        param.push(RequestParam{key: String::from("fromId"), value: from_id.to_string()});
    }

    let resp = client.get_with_param(URL_TRADES_HISTORY, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn agg_trades(client: &Client, symbol: &String, from_id: Option<u64>, start_time: Option<u64>, end_time: Option<u64>, limit: Option<u32>) -> Result<Bytes, String> {
    let mut param = vec![
        RequestParam{key: String::from("symbol"), value: String::from(symbol)},
    ];
    if let Some(from_id) = from_id {
        param.push(RequestParam{key: String::from("fromId"), value: from_id.to_string()});
    }
    if let Some(start_time) = start_time {
        param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
    }
    if let Some(end_time) = end_time {
        param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
    }
    if let Some(mut limit) = limit {
        if limit > 1000 {
            limit = 1000;
        }
        param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
    }

    let resp = client.get_with_param(URL_TRADES_AGG, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn klines(client: &Client, symbol: &String, interval: &String, start_time: Option<u64>, end_time: Option<u64>, limit: Option<u32>) -> Result<Bytes, String> {
    let mut param = vec![
        RequestParam{key: String::from("symbol"), value: String::from(symbol)},
        RequestParam{key: String::from("interval"), value: String::from(interval)},
    ];
    if let Some(start_time) = start_time {
        param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
    }
    if let Some(end_time) = end_time {
        param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
    }
    if let Some(mut limit) = limit {
        if limit > 1000 {
            limit = 1000;
        }
        param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
    }

    let resp = client.get_with_param(URL_KLINES, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn avg_price(client: &Client, symbol: &String) -> Result<Bytes, String> {
    let param = vec![RequestParam{key: String::from("symbol"), value: String::from(symbol)}];
    let resp = client.get_with_param(URL_PRICE_AVG, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn ticker_24hr(client: &Client, symbol: &Option<String>) -> Result<Bytes, String> {
    let mut param = vec![];
    if let Some(symbol) = symbol {
        param.push(RequestParam{key: String::from("symbol"), value: String::from(symbol)});
    }

    let resp = client.get_with_param(URL_TICKER_24HR, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn ticker_price(client: &Client, symbol: &Option<String>) -> Result<Bytes, String> {
    let mut param = vec![];
    if let Some(symbol) = symbol {
        param.push(RequestParam{key: String::from("symbol"), value: String::from(symbol)});
    }

    let resp = client.get_with_param(URL_TICKER_PRICE, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}

pub async fn ticker_book(client: &Client, symbol: &Option<String>) -> Result<Bytes, String> {
    let mut param = vec![];
    if let Some(symbol) = symbol {
        param.push(RequestParam{key: String::from("symbol"), value: String::from(symbol)});
    }

    let resp = client.get_with_param(URL_TICKER_BOOK, &param).await?;
    let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err(err.to_string());
        },
    };
    Ok(body_bytes)
}
