pub mod models;

use std::str;
use hyper::{Client, client::HttpConnector, Request, Method, Body, StatusCode};
use hyper_tls::HttpsConnector;

use self::models::{BskyAuthReq, BskyAuthResp};

pub struct BskyClient {
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub bearer_token: Option<String>,
    pub base_url: String
}

impl BskyClient {
    pub fn new(base_url: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        Self {
            client,
            bearer_token: None,
            base_url
        }
    }

    pub async fn auth(mut self, identifier: String, password: String) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {

        let req = Request::builder()
                            .method(Method::POST)
                            .uri(format!("{}/com.atproto.server.createSession", self.base_url))
                            .header("content-type", "application/json")
                            .header("user-agent", "BOT UCode/0.0.1 https://github.com/Aomitsu/UCode")
                            .body(Body::from(
                                serde_json::to_string(&BskyAuthReq {
                                    identifier,
                                    password
                                })?)
                            )?;
        let res = self.client.request(req).await?;
        if res.status() == StatusCode::OK {
            let body = hyper::body::to_bytes(res).await?;
            let string = str::from_utf8(&body)?;
            let resp: BskyAuthResp = serde_json::from_str(string)?;
            println!("{:?}", resp);
            self.bearer_token = Some(resp.accessJwt);
            //BskyAuthResp::from();
        } else {
            todo!("handle error")
        }

        Ok(self)
    }
    
}