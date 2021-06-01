use crate::binance::{
    msg::*,
    utils::*,
};
use crate::macd::Macd;
use actix::prelude::*;
use actix_broker::BrokerSubscribe;
use reqwest::Method;

pub struct Trader {
    last: Option<Macd>,
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
        if self.last.is_some() && self.last.as_ref().unwrap().divergence * msg.0.divergence < 0.0 {
            // new macd has a different sign (ie. macd and signal series have crossed)
            if msg.0.divergence > 0.0 {
                log::info!("BUY");
            } else {
                log::info!("SELL");
            }
        }

        self.last = Some(msg.0);
    }
}

impl Trader {
    pub fn new() -> Self {
        Self { last: None }
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
        log::info!("{}", response);
        log::info!("{} {} {}", side.as_ref(), quantity, symbol);
        Ok(())
    }
}
