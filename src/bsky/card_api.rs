use hyper::{client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use log::debug;
use serde::{Deserialize, Serialize};
use std::str;
use urlencoding::encode;

pub struct CardyBClient {
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub user_agent: String,
}

impl CardyBClient {
    pub fn new(user_agent: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        debug!("A new CardyB client is created.");

        Self { client, user_agent }
    }

    pub async fn get_card(
        self,
        url: String,
    ) -> Result<CardyBResp, Box<dyn std::error::Error + Send + Sync>> {
        let url_encoded = encode(&url);
        let req = Request::builder()
            .method(Method::GET)
            .uri(format!(
                "https://cardyb.bsky.app/v1/extract?url={}",
                url_encoded
            ))
            .header("user-agent", self.user_agent)
            .body(Body::empty())?;
        let res = self.client.request(req).await?;
        let body = hyper::body::to_bytes(res).await?;
        let string: &str = str::from_utf8(&body)?;
        let resp: CardyBResp = serde_json::from_str(string)?;

        Ok(resp)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardyBResp {
    pub error: String,
    pub likely_type: String,
    pub url: String,
    pub title: String,
    pub description: String,
    pub image: String,
}
