#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BskyAuthReq {
    pub identifier: String,
    pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BskyAuthResp {
    pub did: String,
    pub handle: String,
    pub email: String,
    pub emailConfirmed: bool,
    pub accessJwt: String,
    pub refreshJwt: String,
}

// -- Repo add
#[derive(Serialize, Deserialize, Debug)]
pub struct BskyEmbed {
    #[serde(rename = "$type")]
    pub at_type: String,
    pub external: BskyEmbedExternal,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TextRecord {
    pub text: String,
    pub createdAt: String,
    pub embed: BskyEmbed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BskyEmbedExternal {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub thumb: BskyBlob,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RecordType {
    TextRecord(TextRecord),
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BskyCreateRecordReq {
    pub repo: String,
    pub collection: String,
    pub record: RecordType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BskyPostBlobResp {
    pub blob: BskyBlob,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BskyRef {
    #[serde(rename = "$link")]
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BskyBlob {
    #[serde(rename = "$type")]
    pub at_type: String,
    #[serde(rename = "ref")]
    pub at_ref: BskyRef,
    pub mimeType: String,
    pub size: u32,
}
