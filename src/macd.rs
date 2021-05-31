use ta::{indicators::ExponentialMovingAverage as Ema, Next};

pub struct Macd {
    pub divergence: f64, // difference between macd series and signal series
    macd: f64, // macd series: difference between slow ema and fast ema
    fast_ema: Ema,
    slow_ema: Ema,
    signal_ema: Ema, // signal series: ema of the macd series
}

impl Macd {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            divergence: 0.0,
            macd: 0.0,
            fast_ema: Ema::new(fast_period).unwrap(),
            slow_ema: Ema::new(slow_period).unwrap(),
            signal_ema: Ema::new(signal_period).unwrap(),
        }
    }

    pub fn next(&mut self, val: f64) {
        self.macd = self.slow_ema.next(val) - self.fast_ema.next(val);
        self.divergence = self.signal_ema.next(self.macd) - self.macd;
    }
}
