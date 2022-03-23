pub mod wallet {
    use std::time::{SystemTime, UNIX_EPOCH};

    use hmac::{Hmac, Mac};
    use hyper::body::Bytes;
    use sha2::Sha256;

    use crate::client::client::{Client, RequestParam};

    type HmacSha256 = Hmac<Sha256>;

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
        // timestamp
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
}
