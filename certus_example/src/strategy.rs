use certus_core::core::Order;
use certus_core::strategies::Strategy;

pub struct SimpleStrategy {
    ema_slow: f64,
    ema_fast: f64,
}

impl SimpleStrategy {
    pub fn new() -> Self {
        Self {
            ema_fast: 0.0,
            ema_slow: 0.0,
        }
    }
}

impl Strategy for SimpleStrategy {
    fn init(&mut self) {
        self.ema_slow = 0.0;
        self.ema_fast = 0.0;
    }

    fn next(&mut self) -> Vec<Order> {
        Vec::new()
    }
}
