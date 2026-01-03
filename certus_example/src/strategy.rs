
use certus_core::core::Order;
use certus_core::indicator::{Indicator, MovingAverage};
use certus_core::strategy::{Strategy, StrategyData};
use certus_core::data::MarketData;

pub struct SimpleStrategy {
    data: StrategyData,
    ma_slow: MovingAverage,
    ma_fast: MovingAverage,
}

impl SimpleStrategy {
    pub fn new() -> Self {
        Self {
            data: StrategyData::new(50),
            ma_fast: MovingAverage::new(7),
            ma_slow: MovingAverage::new(21),
        }
    }
}

impl Strategy for SimpleStrategy {
    fn init(&mut self) {
        self.data.init();
    }
    
    fn update(&mut self, market_data: MarketData) {
        self.data.update(market_data);

        self.ma_fast.update(market_data);
        self.ma_slow.update(market_data);
    }

    fn next(&mut self, _market_data: MarketData) -> Vec<Order> {
        if !self.data.is_ready() { return Vec::new(); }

        let current_bar = match self.data.market_data.front() {
            Some(MarketData::Bar(bar)) => bar,
            _ => unreachable!("Expected only MarketData::Bar"),
        };

        println!("Close {} - MA Fast {} - MA Slow {}", current_bar.close, self.ma_fast.value, self.ma_slow.value);

        Vec::new()
    }
}
