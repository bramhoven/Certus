use certus_core::core::Order;
use certus_core::indicator::{Indicator, MovingAverage};
use certus_core::strategy::Strategy;
use certus_core::data::MarketData;

pub struct SimpleStrategy {
    ma_slow: MovingAverage,
    ma_fast: MovingAverage,
}

impl SimpleStrategy {
    pub fn new() -> Self {
        Self {
            ma_fast: MovingAverage::new(7),
            ma_slow: MovingAverage::new(21),
        }
    }
}

impl Strategy for SimpleStrategy {
    fn init(&mut self) {
    }
    
    fn update(&mut self, market_data: MarketData) {
        self.ma_fast.update(market_data);
        self.ma_slow.update(market_data);
    }

    fn next(&mut self, _market_data: MarketData) -> Vec<Order> {
        println!("MA Fast {} - MA Slow {}", self.ma_fast.value, self.ma_slow.value);
        Vec::new()
    }
}
