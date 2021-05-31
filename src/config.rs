use config::{ConfigError, Environment};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub trade: TradeConfig,
    pub macd: MacdConfig,
    pub binance: BinanceConfig,
}

#[derive(Clone, Deserialize)]
pub struct TradeConfig {
    pub symbol: String,
    pub interval: String,
}

#[derive(Clone, Deserialize)]
pub struct MacdConfig {
    pub fast_period: usize,   // default 12
    pub slow_period: usize,   // default 26
    pub signal_period: usize, // default 9
}

#[derive(Clone, Deserialize)]
pub struct BinanceConfig {
    pub api: String,
    pub sec: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }
}
