use crate::binance::{
    msg::*,
    utils::*,
};
use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use actix_rt::Arbiter;
use reqwest::Method;

pub struct Trader {
    symbol: String,
    amount: f64,
    threshold: f64,
    last: Option<f64>, // last macd
}

impl Actor for Trader {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("Trader Actor started");
        self.subscribe_system_async::<MacdUpdate>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        log::info!("Trader Actor stopped");
    }
}

impl Handler<MacdUpdate> for Trader {
    type Result = ();

    fn handle(&mut self, msg: MacdUpdate, _ctx: &mut Self::Context) -> Self::Result {
        let new = msg.0;
        if self.last.is_some() && self.last.unwrap() * new < 0.0 {
            // new macd has a different sign (ie. macd and signal series have crossed)
            let (symbol, amount) = (self.symbol.clone(), self.amount.clone());
            if new >= self.threshold {
                log::info!("BUY");
                // Arbiter::current().spawn(async move {
                //     Self::market_order(symbol, Side::Buy, amount).await.unwrap();
                // });
            } else if new <= -self.threshold {
                log::info!("SELL");
                // Arbiter::current().spawn(async move {
                //     Self::market_order(symbol, Side::Sell, amount).await.unwrap();
                // });
            }
        }

        // only update last if threshold satisfied
        if new >= self.threshold || new <= -self.threshold {
            self.last = Some(new);
        }
    }
}

impl Trader {
    pub fn new(symbol: String, amount: f64, threshold: f64) -> Self {
        Self {
            symbol: symbol,
            amount: amount,
            threshold: threshold,
            last: None,
        }
    }

    pub async fn market_order(symbol: String, side: Side, quantity: f64) -> Result<(), reqwest::Error> {
        let response = signed_req(
            Method::POST,
            "/fapi/v1/order".to_string(),
            format!(
                "symbol={}&side={}&type=MARKET&quantity={}",
                symbol, side.as_ref(), quantity,
            ),
        ).await?;
        log::debug!("{}", response);
        log::info!("{} {} {}", side.as_ref(), quantity, symbol);
        Ok(())
    }

    pub async fn get_qty() -> Result<f64, reqwest::Error> {
        let response = signed_req(
            Method::GET,
            "/fapi/v2/balance".to_string(),
            "".to_string(),
        ).await?;
        log::info!("{}", response);
        Ok(0.0)
    }
}
