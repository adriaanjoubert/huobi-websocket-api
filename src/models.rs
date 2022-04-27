use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ping {
    pub ping: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pong {
    pub pong: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: String,
    pub sub: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionResponse {
    pub id: String,
    pub status: String,
    pub subbed: String,
    pub ts: f32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub req: String,
    pub id: String,
}


#[derive(Debug, Deserialize)]
pub struct HuobiDepthData {
    pub bids: Vec<Vec<f32>>,
    pub asks: Vec<Vec<f32>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HuobiTick {
    pub ch: String,
    pub ts: f32,
    pub tick: HuobiDepthData,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HuobiDepth {
    pub id: String,
    pub status: String,
    pub ts: f32,
    pub rep: String,
    pub data: HuobiDepthData,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum HuobiMessage {
    Tick(HuobiTick),
    Depth(HuobiDepth),
    Ping(Ping),
}
