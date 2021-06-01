use crate::binance::{
    msg::*,
    utils::*,
};
use crate::macd::Macd;
use actix_broker::{Broker, SystemBroker};
use futures::StreamExt;
use reqwest::Method;
use serde_json::{from_str, from_value, Value};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, protocol::Message},
};

pub struct Binance {
    symbol: String,
    interval: String,
    macd: Macd,
    macd_tmp: Macd,
}

impl Binance {
    pub async fn new(
        symbol: String,
        interval: String,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Self {
        let macd = Binance::init_macd(
            symbol.clone(), interval.clone(), fast_period, slow_period, signal_period
        ).await.unwrap();
        Self {
            symbol: symbol,
            interval: interval,
            macd_tmp: macd.clone(),
            macd: macd,
        }
    }

    async fn init_macd(
        symbol: String,
        interval: String,
        fast_period: usize,
        slow_period: usize,
        signal_period: usize,
    ) -> Result<Macd, reqwest::Error> {
        let mut macd = Macd::new(fast_period, slow_period, signal_period);

        let response = unsigned_req(
            Method::GET,
            "/fapi/v1/klines".to_string(),
            format!(
                "symbol={}&interval={}&limit={}",
                symbol, interval, 1500, // use data from past 1499 candles
            ),
        ).await?;

        let klines: Vec<Value> = from_str(&response).unwrap();
        for i in 0..(klines.len() - 1) { // skip last (current) candle
            let close = klines[i][4].as_str().unwrap().parse::<f64>().unwrap();
            macd.next(close);
        }

        Broker::<SystemBroker>::issue_async(MacdUpdate(macd.divergence));
        log::info!("Initialized MACD : {}", macd.divergence);
        Ok(macd)
    }

    pub async fn connect(&mut self) -> Result<(), tungstenite::Error> {
        let uri = format!("{}/ws/{}@kline_{}", BASE_WS, self.symbol.to_lowercase(), self.interval);
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
                        Broker::<SystemBroker>::issue_async(MacdUpdate(self.macd.divergence));
                        log::info!("Updated MACD : {}", self.macd.divergence);
                    } else {
                        self.macd_tmp.next(kline.close);
                        log::info!("Updated curr MACD : {}", self.macd_tmp.divergence);
                        Broker::<SystemBroker>::issue_async(MacdUpdate(self.macd_tmp.divergence));
                    }

                    self.macd_tmp = self.macd.clone();
                    log::debug!("Reset curr MACD : {}", self.macd_tmp.divergence);
                }
                _ => (),
            }
        }

        Ok(())
    }
}
