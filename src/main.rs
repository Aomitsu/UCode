use csv;
use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use std::{env, error::Error, fs::File};

mod bsky;

use crate::bsky::BskyClient;

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

    let client = BskyClient::new("https://bsky.social/xrpc".to_string());
    client.auth(
        env::var("BSKY_USERNAME").unwrap_or("".to_string()),
        env::var("BSKY_PASSWORD").unwrap_or("".to_string()),
    );
}
