use std::fmt;
use ta::{indicators::ExponentialMovingAverage as Ema, Next};

#[derive(Clone)]
pub struct Macd {
    pub macd: f64,
    pub signal: f64,
    pub histogram: f64,
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema,
}

impl Macd {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            macd: 0.,
            signal: 0.,
            histogram: 0.,
            fast_ema: Ema::new(fast_period).unwrap_or(Ema::new(12).unwrap()),
            slow_ema: Ema::new(slow_period).unwrap_or(Ema::new(26).unwrap()),
            signal_ema: Ema::new(signal_period).unwrap_or(Ema::new(9).unwrap()),
        }
    }

    pub fn next(&mut self, val: f64) {
        self.macd = self.fast_ema.next(val) - self.slow_ema.next(val);
        self.signal = self.signal_ema.next(self.macd);
        self.histogram = self.macd - self.signal;
    }
}

impl fmt::Debug for Macd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[macd={} signal={} histogram={}]",
            self.macd, self.signal, self.histogram
        )
    }
}
