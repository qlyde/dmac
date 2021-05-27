use crate::binance::msg::ContinuousKline;
use futures::StreamExt;
use serde_json::{from_str, from_value, Value};
use ta::{indicators::MovingAverageConvergenceDivergence as Macd, Next};
use tokio_tungstenite::{connect_async, tungstenite::{Error, protocol::Message}};

const WS_BASE_URL: &str = "wss://fstream.binance.com";

pub struct Binance {
    pub macd: Macd,
}

impl Binance {
    pub fn new() -> Self {
        Self {
            macd: Macd::default(), // TODO make this variable
        }
    }

    pub async fn connect_kline(&self, symbol: &str, interval: &str) -> Result<(), Error> {
        let uri = format!("{}/ws/{}_perpetual@continuousKline_{}", WS_BASE_URL, symbol, interval);
        log::info!("Connecting to : {}", uri);
        let (mut ws, _) = connect_async(uri).await?;

        while let Some(msg) = ws.next().await {
            let msg = msg?;
            log::debug!("Message received : {:?}", msg);

            match msg {
                Message::Text(txt) => {
                    log::debug!("Text : {}", txt);
                    let mut txt: Value = from_str(&txt).unwrap();
                    let kline: ContinuousKline = from_value(txt["k"].take()).unwrap();
                    log::info!("Price of {} : {}", symbol, kline.close);
                }
                _ => (),
            }
        }

        Ok(())
    }
}
