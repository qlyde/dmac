use actix::prelude::*;
use serde::{de, Deserialize, Deserializer};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct MacdUpdate(f64);

#[derive(Clone, Debug, Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start: u64,
    #[serde(rename = "T")]
    pub end: u64,
    #[serde(rename = "o")]
    #[serde(deserialize_with = "f64_from_str")]
    pub open: f64,
    #[serde(rename = "c")]
    #[serde(deserialize_with = "f64_from_str")]
    pub close: f64,
    #[serde(rename = "h")]
    #[serde(deserialize_with = "f64_from_str")]
    pub high: f64,
    #[serde(rename = "l")]
    #[serde(deserialize_with = "f64_from_str")]
    pub low: f64,
    #[serde(rename = "v")]
    #[serde(deserialize_with = "f64_from_str")]
    pub volume: f64,
    #[serde(rename = "x")]
    pub closed: bool,
}

fn f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}
