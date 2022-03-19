pub mod client {

    use hyper::{Client as HttpClient, client::HttpConnector, Method, Request, Response, Body};

    use std::env;

    pub struct Client {
        api_key: String,
        secret_key: String,

        base_url: &'static str,

        http_client: HttpClient<HttpConnector>,
    }

    impl Client {
        pub fn new() -> Result<Self, String> {
            // try to get ApiKey from env
            let apikey: String;
            let env_apikey = env::var("APIKEY");

            if let Ok(k) = env_apikey {
                apikey = k;
            } else {
                return Err("ERROR: APIKEY not found".to_string());
            }

            let http_client = HttpClient::new();

            Ok(
                Client{
                    api_key: apikey,
                    secret_key: "".into(),
                    base_url: "https://api.binance.com".into(),
                    http_client,
                }
            )
        }

        pub fn set_api_key(&mut self, api_key: String, secret_key: String) {
            self.api_key = api_key;
            self.secret_key = secret_key;
        }

        pub fn read_api_key_from_env() {

        }

        pub fn read_api_key_from_file() {

        }
    }

    impl Client {
        pub async fn get(&self, url: &str) -> Result<Response<Body>, String> {
            let req = Request::builder()
                .method(Method::GET)
                .uri(format!("{}{}", self.base_url, url))
                .body(Body::empty());

            if let Err(_) = req {
                return Err("Body Error".to_string());
            }

            let req = req.ok();

            match req {
                Some(q) => {
                    let resp = self.http_client.request(q).await;
                    if let Ok(r) = resp {
                        Ok(r)
                    } else {
                        Err("Request Error".to_string())
                    }
                },
                _ => Err("Body Error".to_string())
            }
        }
    }
}
