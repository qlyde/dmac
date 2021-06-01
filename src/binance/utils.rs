use crate::config::Config;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{Client, Method};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

pub const BASE_HTTP: &str = "https://fapi.binance.com";
pub const BASE_WS: &str = "wss://fstream.binance.com";

#[derive(AsRefStr)]
pub enum Side {
    #[strum(serialize = "BUY")]
    Buy,
    #[strum(serialize = "SELL")]
    Sell,
}

pub fn digest(message: &[u8], key: &[u8]) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key).unwrap();
    mac.update(message);
    hex::encode(mac.finalize().into_bytes())
}

pub fn timestamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
        .to_string()
}

pub async fn unsigned_req(method: Method, endpoint: String, qstring: String) -> Result<String, reqwest::Error> {
    let config = Config::from_env().unwrap();
    let mut url = format!("{}{}", BASE_HTTP, endpoint);
    if !qstring.is_empty() {
        url.push_str(&format!("?{}", qstring));
    }
    let response = Client::new()
        .request(method, url)
        .header("X-MBX-APIKEY", config.binance.api)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn signed_req(method: Method, endpoint: String, mut qstring: String) -> Result<String, reqwest::Error> {
    let config = Config::from_env().unwrap();

    let ts = timestamp();
    if !qstring.is_empty() {
        qstring.push('&');
    }
    qstring.push_str(&format!("timestamp={}", ts));
    let signature = digest(qstring.as_bytes(), config.binance.sec.as_bytes());

    let url = format!("{}{}?{}&signature={}", BASE_HTTP, endpoint, qstring, signature);
    let response = Client::new()
        .request(method, url)
        .header("X-MBX-APIKEY", config.binance.api)
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn set_leverage(symbol: String, leverage: u64) -> Result<(), reqwest::Error> {
    signed_req(
        Method::POST,
        "/fapi/v1/leverage".to_string(),
        format!(
            "symbol={}&leverage={}",
            symbol, leverage,
        ),
    ).await?;
    Ok(())
}
