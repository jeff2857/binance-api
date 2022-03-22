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
}
