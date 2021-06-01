#[macro_use]
extern crate strum_macros;

pub mod binance;
pub mod config;
pub mod macd;

use crate::binance::{
    info::Binance,
    trader::Trader,
    utils::set_leverage,
};
use crate::config::Config;
use actix::prelude::*;
use actix_rt::{Arbiter, System};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let config = Config::from_env().unwrap();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let sys = System::new();
    sys.block_on(async move {
        set_leverage(config.trade.symbol.clone(), config.trade.leverage).await.unwrap();

        let info = Arbiter::new();
        info.spawn(async {
            Binance::new()
                .await
                .connect(config.trade.symbol, config.trade.interval)
                .await
                .unwrap();
        });

        let trader = Arbiter::new();
        trader.spawn_fn(|| {
            Trader::new().start();
        })
    });

    sys.run().ok();
}
