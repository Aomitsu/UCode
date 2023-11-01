use dotenv::dotenv;
use log::{debug, info};
use std::env;

mod bsky;

use crate::bsky::{card_api::CardyBClient, BskyClient};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Hello, world!");

    // TODO: Avoid to use unwrap

    let client: BskyClient = BskyClient::new(
        "https://bsky.social/xrpc".to_string(),
        env::var("USER_AGENT").unwrap_or("UBot/Fallback user_agent".to_string()),
    );

    let card_client: CardyBClient =
        CardyBClient::new(env::var("USER_AGENT").unwrap_or("UBot/Fallback user_agent".to_string()));

    let card = card_client
        .get_card("https://github.com/Aomitsu/UCode".to_string())
        .await
        .unwrap();

    debug!("Card : {:?}", card);
    let authed_client = client
        .auth(
            env::var("BSKY_USERNAME").unwrap_or("".to_string()),
            env::var("BSKY_PASSWORD").unwrap_or("".to_string()),
        )
        .await
        .unwrap();
    authed_client
        .send_message(
            card.description,
            card.image,
            "https://github.com/Aomitsu/UCode".to_string(),
            card.title,
        )
        .await
        .unwrap();
}
