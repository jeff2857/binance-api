pub mod client {

    use hyper_proxy::{Proxy, Intercept, ProxyConnector};
    use hyper_tls::HttpsConnector;
    use log::info;

    use hyper::{Client as HttpClient, client::HttpConnector, Method, Request, Response, Body, Uri};

    use std::{env, str::FromStr};

    #[derive(Debug)]
    enum EClient {
        Client(HttpClient<HttpsConnector<HttpConnector>>),
        ProxyClient(HttpClient<ProxyConnector<HttpsConnector<HttpConnector>>>),
    }

    #[derive(Debug)]
    pub struct Client {
        api_key: String,
        secret_key: String,

        base_url: &'static str,

        http_client: EClient,
        proxy: Option<ProxyConnector<HttpsConnector<HttpConnector>>>,
    }

    impl Client {
        pub fn new() -> Result<Self, String> {
            env_logger::init();

            let http_client = HttpClient::builder().build::<_, hyper::Body>(HttpsConnector::new());

            let mut client = Client {
                api_key: "".to_string(),
                secret_key: "".to_string(),
                base_url: &"https://api.binance.com",
                http_client: EClient::Client(http_client),
                proxy: None,
            };

            match client.get_api_key() {
                Ok(_) => {return Ok(client);},
                Err(e) => Err(e),
            }
        }

        pub fn with_proxy(proxy_uri: String) -> Result<Self, String> {
            let proxy = {
                let proxy = Proxy::new(Intercept::All, proxy_uri.parse().unwrap());
                let connector = HttpsConnector::new();
                let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
                proxy_connector
            };

            let http_client = HttpClient::builder().build::<_, hyper::Body>(proxy.clone());
            let mut client = Client {
                api_key: "".to_string(),
                secret_key: "".to_string(),
                base_url: &"https://api.binance.com",
                http_client: EClient::ProxyClient(http_client),
                proxy: Some(proxy),
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
                //.uri("https://www.baidu.com".to_string())
                .body(Body::empty());

            if let Err(_) = req {
                return Err("Body Error".to_string());
            }

            let req = req.ok();

            match req {
                Some(mut q) => {
                    info!("{:?}", &q);
                    if let Some(proxy) = &self.proxy {
                        if let Some(headers) = proxy.http_headers(&Uri::from_str(url).unwrap()) {
                            q.headers_mut().extend(headers.clone().into_iter());
                        }
                    }

                    match &self.http_client {
                        EClient::Client(client) => {
                            let resp = client.request(q).await;
                            if let Ok(r) = resp {
                                Ok(r)
                            } else {
                                Err("Request Error".to_string())
                            }
                        },
                        EClient::ProxyClient(client) => {
                            let resp = client.request(q).await;
                            if let Ok(r) = resp {
                                Ok(r)
                            } else {
                                Err("Request Error".to_string())
                            }

                        }
                    }

                },
                _ => Err("Body Error".to_string())
            }
        }
    }
}
