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
pub struct SimpleTextRecord {
    pub text: String,
    pub createdAt: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RecordType {
    SimpleText(SimpleTextRecord),
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
    #[serde(alias = "$link")]
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BskyBlob {
    #[serde(alias = "$type")]
    pub at_type: String,
    #[serde(alias = "ref")]
    pub at_ref: BskyRef,
    pub mimeType: String,
    pub size: u32,
}
