use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
