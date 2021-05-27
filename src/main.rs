pub mod binance;
pub mod config;

use crate::binance::exec::Binance;
use crate::config::Config;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::from_env().unwrap();

    env::set_var("RUST_LOG", "info");
    env_logger::init();

    Binance::new().connect_kline(&config.trade.symbol, &config.trade.interval).await.unwrap();
}
