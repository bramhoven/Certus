use certus_core::broker::Broker;
use certus_core::core::{Order, OrderSide, OrderType};
use certus_core::data::MarketData;
use certus_core::indicator::{Indicator, MovingAverage};
use certus_core::strategy::{Strategy, StrategyData};

use log;

pub struct SimpleStrategy {
    id: usize,
    instrument: u32,
    data: StrategyData,
    ma_slow: MovingAverage,
    ma_fast: MovingAverage,
}

impl SimpleStrategy {
    pub fn new(instrument: u32) -> Self {
        Self {
            id: 0,
            instrument,
            data: StrategyData::new(50),
            ma_fast: MovingAverage::new(7, 50),
            ma_slow: MovingAverage::new(21, 50),
        }
    }
}

impl Strategy for SimpleStrategy {
    fn init(&mut self, id: usize) {
        self.id = id;
        self.data.init(0);
    }

    fn update(&mut self, market_data: MarketData, broker: &mut dyn Broker) {
        self.data.update(market_data, broker);

        self.ma_fast.update(market_data);
        self.ma_slow.update(market_data);
    }

    fn next(&mut self, _market_data: MarketData, broker: &mut dyn Broker) {
        if !self.data.is_ready() {
            return;
        }

        let current_bar = match self.data.market_data.front() {
            Some(MarketData::Bar(bar)) => bar,
            _ => unreachable!("Expected only MarketData::Bar"),
        };

        log::debug!(
            "Close {} - MA Fast {} - MA Slow {}",
            current_bar.close,
            self.ma_fast.value,
            self.ma_slow.value
        );

        // Crossover upside check
        if self.ma_fast[1] < self.ma_slow[1] && self.ma_fast[0] > self.ma_slow[0] {
            log::info!("Crossover UP");
            let _order = broker.place_order(Order {
                id: None,
                instrument: self.instrument,
                strategy_id: self.id,
                side: OrderSide::Buy,
                order_type: OrderType::Market,
                size: 1.0,
            });
        }

        // Crossover downside check
        if self.ma_fast[1] > self.ma_slow[1] && self.ma_fast[0] < self.ma_slow[0] {
            log::info!("Crossover DOWN");
            let _order = broker.place_order(Order {
                id: None,
                instrument: self.instrument,
                strategy_id: self.id,
                side: OrderSide::Sell,
                order_type: OrderType::Market,
                size: 1.0,
            });
        }
    }
}
