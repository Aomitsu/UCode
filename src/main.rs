use dotenv::dotenv;
use log::{debug, info};
use rand::prelude::*;
use std::env;

mod bsky;
mod json;

use crate::{
    bsky::{card_api::CardyBClient, BskyClient},
    json::{JsonData, JsonStructure},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    info!("Hello, world!");

    let datajs = json::read_from_file("data.json");

    // TODO: Avoid to use unwrap

    let client: BskyClient = BskyClient::new(
        "https://bsky.social/xrpc".to_string(),
        env::var("USER_AGENT").unwrap_or("UBot/Fallback user_agent".to_string()),
    );

    let card_client: CardyBClient =
        CardyBClient::new(env::var("USER_AGENT").unwrap_or("UBot/Fallback user_agent".to_string()));

    let authed_client = client
        .auth(
            env::var("BSKY_USERNAME").unwrap_or("".to_string()),
            env::var("BSKY_PASSWORD").unwrap_or("".to_string()),
        )
        .await
        .unwrap();

    let _ = send_message(datajs, authed_client, card_client).await;
}

pub async fn send_message(
    datajs: JsonData,
    client: BskyClient,
    card_client: CardyBClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // pick random entry into datajs
    let mut random_index = rand::thread_rng();
    let random_entry: Option<&JsonStructure> =
        datajs.0.get(random_index.gen_range(0..datajs.0.len()));
    if let Some(entry) = random_entry {
        let card = card_client
            .get_card(entry.repository_url.as_ref().unwrap().to_string())
            .await
            .unwrap();

        debug!("{:?}", card);

        let description = entry
            .description
            .as_ref()
            .unwrap_or(&"Aucune description fournie sur code.gouv.fr".to_string())
            .to_string();
        let entity = entry
            .organization_name
            .as_ref()
            .unwrap_or(&"unknown".to_string())
            .to_string();
        let project_name = entry
            .name
            .as_ref()
            .unwrap_or(&"unknown".to_string())
            .to_string();

        client
            .send_message(
                format!("{}/{}\n{}", entity, project_name, description),
                card.image,
                entry.repository_url.as_ref().unwrap().to_string(),
                card.title,
            )
            .await
            .unwrap();
    };
    Ok(())
}
