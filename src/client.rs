pub mod client {

    use log::info;

    use hyper::{Client as HttpClient, client::HttpConnector, Method, Request, Response, Body};

    use std::env;

    #[derive(Debug)]
    pub struct Client {
        api_key: String,
        secret_key: String,

        base_url: &'static str,

        http_client: HttpClient<HttpConnector>,
    }

    impl Client {
        pub fn new() -> Result<Self, String> {
            env_logger::init();

            let mut client = Client {
                api_key: "".to_string(),
                secret_key: "".to_string(),
                base_url: &"https://api.binance.com",
                http_client: HttpClient::new(),
            };

            match client.get_api_key() {
                Ok(_) => {return Ok(client);},
                Err(e) => Err(e),
            }
        }

        fn get_api_key(&mut self) -> Result<(), String> {
            let api_key = self.get_api_key_from_env();
            if let Ok(k) = api_key {
                self.api_key = k;
            } else {
                return Err("APIKEY not found".to_string());
            }

            let secret_key = self.get_secret_key_from_env();
            if let Ok(k) = secret_key {
                self.secret_key = k;
            } else {
                return Err("SECRETKEY not found".to_string());
            }

            Ok(())
        }

        fn get_api_key_from_env(&self) -> Result<String, String> {
            info!("Trying to get APIKEY from env");

            let env_apikey = env::var("APIKEY");

            if let Ok(k) = env_apikey {
                Ok(k)
            } else {
                Err("APIKEY not found".to_string())
            }
        }

        fn get_secret_key_from_env(&self) -> Result<String, String> {
            info!("Trying to get SECRETKEY from env");

            let env_secretkey = env::var("SECRETKEY");

            if let Ok(k) = env_secretkey {
                Ok(k)
            } else {
                Err("SECRETKEY not found".to_string())
            }
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
