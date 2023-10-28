use dotenv::dotenv;
use serde::{Serialize, Deserialize};
use std::{fs::File, error::Error, str};
use csv;

mod bsky;

#[derive(Debug, Deserialize)]
pub struct Record {
    description: String,
    name: String,
    repository_url: String
}

fn parse_projects () -> Result<(), Box<dyn Error>> {
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
    //let _ = request().await;
    let _ = parse_projects();

    // let test = BskyClient::new("https://bsky.social/xrpc".to_string());
    //let _ = test.auth("".to_string(), "".to_string()).await;
}

