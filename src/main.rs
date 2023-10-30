use csv;
use dotenv::dotenv;
use log::{info, debug};
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs::File};

mod bsky;

use crate::bsky::{BskyClient, card_api::CardyBClient};

#[derive(Debug, Deserialize)]
pub struct Record {
    description: String,
    name: String,
    repository_url: String,
}

fn parse_projects() -> Result<(), Box<dyn Error>> {
    let file_path = "data.csv";
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        println!("{:?}", result);
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

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

    let card_client: CardyBClient = CardyBClient::new(
        env::var("USER_AGENT").unwrap_or("UBot/Fallback user_agent".to_string()),
    );

    let card = card_client.get_card("https://github.com/Aomitsu/UCode".to_string()).await.unwrap();

    debug!("Card : {:?}", card);
    /*let authed_client = client.auth(
        env::var("BSKY_USERNAME").unwrap_or("".to_string()),
        env::var("BSKY_PASSWORD").unwrap_or("".to_string()),
    ).await.unwrap();
    authed_client.send_simple_message("Ceci est un message de test".to_string()).await.unwrap();*/


}
