pub mod market {
    use hyper::body::Bytes;

    use crate::client::client::{Client, RequestParam};

    pub async fn ping(client: &Client) -> Result<Bytes, String> {
        let uri = &"/api/v3/ping";
        let resp = client.get(uri).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        Ok(body_bytes)
    }

    pub async fn time(client: &Client) -> Result<Bytes, String> {
        let uri = &"/api/v3/time";
        let resp = client.get(uri).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        Ok(body_bytes)
    }

    pub async fn exchange_info(client: &Client) -> Result<Bytes, String> {
        let uri = &"/api/v3/exchangeInfo";
        let resp = client.get(uri).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        Ok(body_bytes)
    }

    pub async fn exchange_info_symbol(client: &Client, symbol: &String) -> Result<Bytes, String> {
        let uri = &"/api/v3/exchangeInfo";
        let param = vec![RequestParam{key: String::from("symbol"), value: String::from(symbol)}];
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };
        Ok(body_bytes)
    }

    pub async fn exchange_info_symbols(client: &Client, symbols: &Vec<String>) -> Result<Bytes, String> {
        let uri = &"/api/v3/exchangeInfo";

        let mut symbols_str = String::from("[");
        for s in symbols.iter() {
            symbols_str.push_str(s.as_str());
        }
        symbols_str.push_str(&"]");

        let param = vec![RequestParam{key: String::from("symbol"), value: symbols_str}];
        let resp = client.get_with_param(uri, &param).await?;
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

        let uri = &"/api/v3/depth";

        let param = vec![
            RequestParam{key: String::from("symbol"), value: String::from(symbol)},
            RequestParam{key: String::from("limit"), value: limit.to_string()},
        ];
        let resp = client.get_with_param(uri, &param).await?;
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

        let uri = &"/api/v3/trades";

        let param = vec![
            RequestParam{key: String::from("symbol"), value: String::from(symbol)},
            RequestParam{key: String::from("limit"), value: limit.to_string()},
        ];
        let resp = client.get_with_param(uri, &param).await?;
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

        let uri = &"/api/v3/historicalTrades";

        let mut param = vec![
            RequestParam{key: String::from("symbol"), value: String::from(symbol)},
            RequestParam{key: String::from("limit"), value: limit.to_string()},
        ];
        if let Some(from_id) = from_id {
            param.push(RequestParam{key: String::from("fromId"), value: from_id.to_string()});
        }

        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };
        Ok(body_bytes)
    }

    pub async fn agg_trades(client: &Client, symbol: &String, from_id: Option<u64>, start_time: Option<u64>, end_time: Option<u64>, limit: Option<u32>) -> Result<Bytes, String> {
        let uri = &"/api/v3/aggTrades";

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

        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };
        Ok(body_bytes)
    }
}
