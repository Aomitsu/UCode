pub mod card_api;
pub mod models;

use chrono::{DateTime, Utc};
use hyper::{client::HttpConnector, Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use log::debug;
use std::{str, time::SystemTime};

use crate::bsky::models::{BskyEmbed, BskyEmbedExternal, BskyPostBlobResp, TextRecord};

use self::models::{BskyAuthReq, BskyAuthResp, BskyCreateRecordReq, RecordType};

#[derive(Clone, Debug)]
pub struct BskyClient {
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub bearer_token: Option<String>,
    pub base_url: String,
    pub user_agent: String,
    pub user: Option<BskyUser>,
}
#[derive(Clone, Debug)]
pub struct BskyUser {
    pub did: String,
    pub handle: String,
}

const CONTENT_TYPE: &str = "application/json";

impl BskyClient {
    pub fn new(base_url: String, user_agent: String) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        debug!("A new bluesky client is created.");

        Self {
            client,
            bearer_token: None,
            base_url,
            user_agent,
            user: None,
        }
    }

    pub async fn auth(
        mut self,
        identifier: String,
        password: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let data = self.clone();
        let req = Request::builder()
            .method(Method::POST)
            .uri(format!(
                "{}/com.atproto.server.createSession",
                self.base_url
            ))
            .header("content-type", CONTENT_TYPE)
            .header("user-agent", data.user_agent)
            .body(Body::from(serde_json::to_string(&BskyAuthReq {
                identifier,
                password,
            })?))?;

        let res = self.client.request(req).await?;
        if res.status() == StatusCode::OK {
            let body = hyper::body::to_bytes(res).await?;
            let string = str::from_utf8(&body)?;
            let resp: BskyAuthResp = serde_json::from_str(string)?;
            self.bearer_token = Some(resp.accessJwt);
            self.user = Some(BskyUser {
                did: resp.did,
                handle: resp.handle,
            });
            debug!("Bluesky client is authenticated.");
            //BskyAuthResp::from();
        } else {
            debug!("Bluesky client is not authenticated.");
            panic!("Bluesky client is not authenticated.")
        }

        Ok(self)
    }

    pub async fn send_image(
        self,
        image_url: String,
    ) -> Result<BskyPostBlobResp, Box<dyn std::error::Error + Send + Sync>> {
        let data = self.clone();

        let img_req = Request::builder()
            .method(Method::GET)
            .uri(image_url)
            .body(Body::empty())?;
        let mut res_image = self.client.request(img_req).await?;

        let image_body = hyper::body::to_bytes(res_image.body_mut()).await?;
        let image_mime = res_image.headers().get("content-type").unwrap().to_str()?;

        let post_img_req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/com.atproto.repo.uploadBlob", data.base_url))
            .header("content-type", image_mime)
            .header("user-agent", data.user_agent)
            .header(
                "Authorization",
                format!("Bearer {}", data.bearer_token.unwrap()),
            )
            .body(Body::from(image_body))?;

        let res_post_image = self.client.request(post_img_req).await?;
        if res_post_image.status() == StatusCode::OK {
            let body = hyper::body::to_bytes(res_post_image).await?;
            let string = str::from_utf8(&body)?;
            let resp: BskyPostBlobResp = serde_json::from_str(string)?;
            debug!("Bluesky client just uploaded an image.");
            Ok(resp)
            //BskyAuthResp::from();
        } else {
            debug!("Bluesky client can't upload image.");
            panic!("Bluesky client can't upload image.")
        }
    }

    pub async fn send_message(
        self,
        text: String,
        image_link: String,
        uri: String,
        title: String,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        let data = self.clone();

        let image = self.clone().send_image(image_link).await?;

        let body = BskyCreateRecordReq {
            repo: self.clone().user.unwrap().handle.to_string(),
            collection: "app.bsky.feed.post".to_string(),
            record: RecordType::TextRecord(TextRecord {
                text: text.clone(),
                createdAt: now,
                embed: BskyEmbed {
                    at_type: "app.bsky.embed.external".to_string(),
                    external: BskyEmbedExternal {
                        uri: uri.to_string(),
                        title,
                        description: text,
                        thumb: image.blob,
                    },
                },
            }),
        };

        let req = Request::builder()
            .method(Method::POST)
            .uri(format!("{}/com.atproto.repo.createRecord", data.base_url))
            .header("content-type", CONTENT_TYPE)
            .header("user-agent", data.user_agent)
            .header(
                "Authorization",
                format!("Bearer {}", data.bearer_token.unwrap()),
            )
            .body(Body::from(serde_json::to_string(&body)?))?;
        let res = self.client.request(req).await?;

        if res.status() == StatusCode::OK {
            debug!("Bluesky client just sent a message.");
            let bady = hyper::body::to_bytes(res).await?;

            debug!("Message: {:?}", serde_json::to_string(&body)?);
            debug!("Result: {:?}", str::from_utf8(&bady)?);
            Ok(self)
        } else {
            debug!("Bluesky client can't send message.");
            panic!("Bluesky client can't send message.")
        }
    }
}
