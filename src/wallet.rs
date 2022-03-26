pub mod wallet {
    use std::{time::{SystemTime, UNIX_EPOCH}, fmt};

    use hmac::{Hmac, Mac};
    use hyper::body::Bytes;
    use sha2::Sha256;

    use crate::client::client::{Client, RequestParam};

    type HmacSha256 = Hmac<Sha256>;

    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum EAssetTransferType {
        MAIN_UMFUTURE,
        MAIN_CMFUTURE,
        MAIN_MARGIN,
        UMFUTURE_MAIN,
        UMFUTURE_MARGIN,
        CMFUTURE_MAIN,
        MARGIN_MAIN,
        MARGIN_UMFUTURE,
        MARGIN_CMFUTURE,
        CMFUTURE_MARGIN,
        ISOLATEDMARGIN_MARGIN,
        MARGIN_ISOLATEDMARGIN,
        ISOLATEDMARGIN_ISOLATEDMARGIN,
        MAIN_FUNDING,
        FUNDING_MAIN,
        FUNDING_UMFUTURE,
        UMFUTURE_FUNDING,
        MARGIN_FUNDING,
        FUNDING_MARGIN,
        FUNDING_CMFUTURE,
        CMFUTURE_FUNDING,
    }

    impl fmt::Display for EAssetTransferType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    fn get_timestamp() -> u64 {
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).unwrap();
        ts.as_secs() * 1000
    }

    fn get_signature(param: &String, secret: &String) -> String {
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC Error".into());
        mac.update(param.as_bytes());
        let result_slice = &mac.finalize().into_bytes()[..];
        let result = hex::encode(result_slice);
        result
    }

    fn param2string(param: &Vec<RequestParam>) -> String {
        let mut param_str = String::new();
        for p in param {
            param_str.push_str(&format!("{}={}&", &p.key, &p.value));
        }
        param_str.remove(param_str.len() - 1);
        param_str
    }

    pub async fn system_status(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/system/status";

        let resp = client.get(uri).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn capital_all(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/capital/config/getall";

        let timestamp = get_timestamp();
        let mut param = vec![
            RequestParam{key: String::from("timestamp"), value: timestamp.to_string()},
        ];

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        println!("signature: {}", &signature);
        param.push(RequestParam{key: String::from("signature"), value: signature});

        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn account_snapshot(client: &Client, account_type: &String, start_time: Option<u64>, end_time: Option<u64>, limit: Option<u32>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/accountSnapshot";

        let mut param = vec![
            RequestParam{key: String::from("type"), value: String::from(account_type)},
        ];
        if let Some(start_time) = start_time {
            param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
        }
        if let Some(end_time) = end_time {
            param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
        }
        if let Some(limit) = limit {
            param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
        }

        let timestamp = get_timestamp();
        param.push(RequestParam{key: String::from("timestamp"), value: timestamp.to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});

        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_dust_btc(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/dust-btc";

        let mut param = vec![];

        let timestamp = get_timestamp();
        param.push(RequestParam{key: String::from("timestamp"), value: timestamp.to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn disable_fast_withdraw_switch(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/account/disableFastWithdrawSwitch";

        let mut param = vec![];

        let timestamp = get_timestamp();
        param.push(RequestParam{key: String::from("timestamp"), value: timestamp.to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn enable_fast_withdraw_switch(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/account/enableFastWithdrawSwitch";

        let mut param = vec![];

        let timestamp = get_timestamp();
        param.push(RequestParam{key: String::from("timestamp"), value: timestamp.to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn capital_withdraw(
        client: &Client,
        coin: &str,
        amount: f64,
        address: &str,
        withdraw_order_id: &Option<&str>,
        network: &Option<&str>,
        address_tag: &Option<&str>,
        transaction_fee_flag: &Option<bool>,
        name: &Option<&str>,
        wallet_type: &Option<u8>
    ) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/capital/withdraw/apply";

        let mut param = vec![
            RequestParam{key: String::from("coin"), value: String::from(coin)},
            RequestParam{key: String::from("address"), value: String::from(address)},
            RequestParam{key: String::from("amount"), value: amount.to_string()},
        ];

        if let Some(withdraw_order_id) = withdraw_order_id {
            param.push(RequestParam{key: String::from("withdrawOrderId"), value: String::from(*withdraw_order_id)});
        }
        if let Some(network) = network {
            param.push(RequestParam{key: String::from("network"), value: String::from(*network)});
        }
        if let Some(address_tag) = address_tag {
            param.push(RequestParam{key: String::from("addressTag"), value: String::from(*address_tag)});
        }
        if let Some(transaction_fee_flag) = transaction_fee_flag {
            param.push(RequestParam{key: String::from("transactionFeeFlag"), value: transaction_fee_flag.to_string()});
        }
        if let Some(name) = name {
            param.push(RequestParam{key: String::from("name"), value: String::from(*name)});
        }
        if let Some(wallet_type) = wallet_type {
            param.push(RequestParam{key: String::from("walletType"), value: wallet_type.to_string()});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn capital_deposit_hisrec(
        client: &Client,
        coin: &Option<&str>,
        status: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        offset: Option<i32>,
        limit: Option<u32>,
    ) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/capital/deposit/hisrec";

        let mut param = vec![];

        if let Some(coin) = coin {
            param.push(RequestParam{key: String::from("coin"), value: String::from(*coin)});
        }
        if let Some(status) = status {
            param.push(RequestParam{key: String::from("status"), value: status.to_string()});
        }
        if let Some(start_time) = start_time {
            param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
        }
        if let Some(end_time) = end_time {
            param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
        }
        if let Some(offset) = offset {
            param.push(RequestParam{key: String::from("offset"), value: offset.to_string()});
        }
        if let Some(limit) = limit {
            param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn capital_withdraw_history(
        client: &Client,
        coin: &Option<&str>,
        withdraw_order_id: &Option<&str>,
        status: Option<u32>,
        offset: Option<i32>,
        limit: Option<u32>,
        start_time: Option<u64>,
        end_time: Option<u64>
    ) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/capital/withdraw/history";

        let mut param = vec![];

        if let Some(coin) = coin {
            param.push(RequestParam{key: String::from("coin"), value: String::from(*coin)});
        }
        if let Some(withdraw_order_id) = withdraw_order_id {
            param.push(RequestParam{key: String::from("withdrawOrderId"), value: String::from(*withdraw_order_id)});
        }
        if let Some(status) = status {
            param.push(RequestParam{key: String::from("status"), value: status.to_string()});
        }
        if let Some(start_time) = start_time {
            param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
        }
        if let Some(end_time) = end_time {
            param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
        }
        if let Some(offset) = offset {
            param.push(RequestParam{key: String::from("offset"), value: offset.to_string()});
        }
        if let Some(limit) = limit {
            param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn capital_deposit_address(client: &Client, coin: &str, network: &Option<&str>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/capital/deposit/address";

        let mut param = vec![
            RequestParam{key: String::from("coin"), value: String::from(coin)},
        ];

        if let Some(network) = network {
            param.push(RequestParam{key: String::from("network"), value: String::from(*network)});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn account_status(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/account/status";

        let mut param = vec![
            RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()},
        ];

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn account_api_trading_status(client: &Client) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/account/apiTradingStatus";

        let mut param = vec![
            RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()},
        ];

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_dribblet(client: &Client, start_time: Option<u64>, end_time: Option<u64>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/dribblet";

        let mut param = vec![];

        if let Some(start_time) = start_time {
            param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
        }
        if let Some(end_time) = end_time {
            param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_dust(client: &Client, asset: &Vec<&str>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/dust";

        let mut asset_str = String::new();
        for a in asset {
            asset_str.push_str(format!("&asset={}", a).as_str());
        }
        asset_str.remove(0);

        let mut param = vec![
            RequestParam{key: String::from("asset"), value: asset_str},
        ];

        let timestamp = get_timestamp();
        param.push(RequestParam{key: String::from("timestamp"), value: timestamp.to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_dividend(client: &Client, asset: &Option<&str>, start_time: Option<u64>, end_time: Option<u64>, limit: Option<u32>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/assetDividend";

        let mut param = vec![];

        if let Some(asset) = asset {
            param.push(RequestParam{key: String::from("asset"), value: String::from(*asset)});
        }
        if let Some(start_time) = start_time {
            param.push(RequestParam{key: String::from("startTime"), value: start_time.to_string()});
        }
        if let Some(end_time) = end_time {
            param.push(RequestParam{key: String::from("endTime"), value: end_time.to_string()});
        }
        if let Some(limit) = limit {
            param.push(RequestParam{key: String::from("limit"), value: limit.to_string()});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_detail(client: &Client, asset: &Option<&str>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/assetDetail";

        let mut param = vec![];

        if let Some(asset) = asset {
            param.push(RequestParam{key: String::from("asset"), value: String::from(*asset)});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_trade_fee(client: &Client, symbol: &Option<&str>) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/tradeFee";

        let mut param = vec![];

        if let Some(symbol) = symbol {
            param.push(RequestParam{key: String::from("symbol"), value: String::from(*symbol)});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.get_with_param(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }

    pub async fn asset_transfer(
        client: &Client,
        transfer_type: EAssetTransferType,
        asset: &str,
        amount: f64,
        from_symbol: &Option<&str>,
        to_symbol: &Option<&str>
    ) -> Result<Bytes, String> {
        let uri = &"/sapi/v1/asset/transfer";

        let mut param = vec![
            RequestParam{key: String::from("type"), value: transfer_type.to_string()},
            RequestParam{key: String::from("asset"), value: String::from(asset)},
            RequestParam{key: String::from("amount"), value: amount.to_string()},
        ];

        if let Some(from_symbol) = from_symbol {
            param.push(RequestParam{key: String::from("fromSymbol"), value: String::from(*from_symbol)});
        }
        if let Some(to_symbol) = to_symbol {
            param.push(RequestParam{key: String::from("toSymbol"), value: String::from(*to_symbol)});
        }
        param.push(RequestParam{key: String::from("timestamp"), value: get_timestamp().to_string()});

        let param_str = param2string(&param);
        let signature = get_signature(&param_str, client.get_secret_key());
        param.push(RequestParam{key: String::from("signature"), value: signature});
    
        let resp = client.post(uri, &param).await?;
        let body_bytes = match hyper::body::to_bytes(resp.into_body()).await {
            Ok(bytes) => bytes,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        Ok(body_bytes)
    }
}
