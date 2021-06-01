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
        // set account leverage for trade symbol
        set_leverage(config.trade.symbol.clone(), config.trade.leverage).await.unwrap();

        let cfg = config.clone();
        let info = Arbiter::new();
        info.spawn(async {
            Binance::new(
                cfg.trade.symbol,
                cfg.trade.interval,
                cfg.macd.fast_period,
                cfg.macd.slow_period,
                cfg.macd.signal_period,
            ).await.connect().await.unwrap();
        });

        let cfg = config.clone();
        let trader = Arbiter::new();
        trader.spawn_fn(|| {
            Trader::new(
                cfg.trade.symbol,
                cfg.trade.amount,
                cfg.trade.threshold,
            ).start();
        })
    });

    sys.run().ok();
}
