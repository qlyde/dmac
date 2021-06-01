use crate::binance::{
    msg::*,
    utils::*,
};
use crate::config::Config;
use crate::macd::Macd;
use futures::StreamExt;
use reqwest::Method;
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, protocol::Message},
};

pub struct Binance {
    pub macd: Macd,
}

impl Binance {
    pub async fn new() -> Self {
        Self {
            macd: Binance::init_macd().await.unwrap(),
        }
    }

    async fn init_macd() -> Result<Macd, reqwest::Error> {
        let config = Config::from_env().unwrap();
        let mut macd = Macd::new(
            config.macd.fast_period,
            config.macd.slow_period,
            config.macd.signal_period,
        );

        let response = unsigned_req(
            Method::GET,
            "/fapi/v1/klines".to_string(),
            format!(
                "symbol={}&interval={}&limit={}",
                config.trade.symbol, config.trade.interval, 1500,
            ),
        )
        .await?;

        let klines: Vec<Value> = from_str(&response).unwrap();
        for i in 0..(klines.len() - 1) { // skip last (current) candle
            let close = klines[i][4].as_str().unwrap().parse::<f64>().unwrap();
            macd.next(close);
        }

        log::info!("Initialized MACD : {}", macd.divergence);
        Ok(macd)
    }

    pub async fn connect(&mut self, symbol: String, interval: String) -> Result<(), tungstenite::Error> {
        let uri = format!("{}/ws/{}@kline_{}", BASE_WS, symbol, interval);
        log::info!("Connecting to : {}", uri);
        let (mut ws, _) = connect_async(uri).await?;

        while let Some(msg) = ws.next().await {
            let msg = msg?;
            log::debug!("Message received : {:?}", msg);

            match msg {
                Message::Text(txt) => {
                    let mut txt: Value = from_str(&txt).unwrap();
                    let kline: Kline = from_value(txt["k"].take()).unwrap();
                    log::debug!("{:?}", kline);

                    if kline.closed {
                        self.macd.next(kline.close);
                        log::info!("Updated MACD : {}", self.macd.divergence);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
