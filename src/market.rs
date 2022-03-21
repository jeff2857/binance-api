pub mod market {
    use hyper::body::Bytes;

    use crate::client::client::Client;

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
}
