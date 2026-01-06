use certus_core::broker::Broker;
use certus_core::core::{OrderSide, OrderType};
use certus_core::data::MarketData;
use certus_core::indicator::{Indicator, MovingAverage};
use certus_core::strategy::{Strategy, MarketDataReceiver, StrategyBase, StrategyData};

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

impl MarketDataReceiver for SimpleStrategy {
    fn update(&mut self, market_data: MarketData, broker: &mut dyn Broker) {
        self.data.update(market_data, broker);

        self.ma_fast.update(market_data);
        self.ma_slow.update(market_data);
    }
}

impl StrategyBase for SimpleStrategy {
    fn init(&mut self, id: usize) {
        self.id = id;
    }
    
    fn get_id(&self) -> usize {
        self.id
    }
    
    fn get_instrument(&self) -> u32 {
        self.instrument
    }
}

impl Strategy for SimpleStrategy {
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

        let cur_pos = broker.get_current_position(self.id, self.instrument);
        let mut cur_trade_id: Option<usize> = None;
        if cur_pos != 0.0 {
            let trades = broker.get_open_trades(self.id, self.instrument);
            if trades.len() > 0 {
                cur_trade_id = Some(trades.first().map(|t| t.id).unwrap());
            }
        }

        // Check for entry
        if cur_pos == 0.0 {
            // Crossover upside check
            if self.ma_fast[1] < self.ma_slow[1] && self.ma_fast[0] > self.ma_slow[0] {
                log::info!("[ENTRY] Crossover UP");
                let _order_id = self.place_order(broker, OrderSide::Buy, OrderType::Market, 1.0);
            }

            // Crossover downside check
            if self.ma_fast[1] > self.ma_slow[1] && self.ma_fast[0] < self.ma_slow[0] {
                log::info!("[ENTRY] Crossover DOWN");
                let _order_id = self.place_order(broker, OrderSide::Sell, OrderType::Market, 1.0);
            }
        }

        // Check for exiting long
        else if cur_pos > 0.0 {
            // Crossover downside check
            if self.ma_fast[1] > self.ma_slow[1] && self.ma_fast[0] < self.ma_slow[0] {
                log::info!("[EXIT] Crossover DOWN");
                let _order_id = self.place_related_order(broker, OrderSide::Buy, OrderType::Market, 1.0, cur_trade_id.unwrap());
            }
        }

        // Check for exiting short
        else if cur_pos < 0.0 {
            // Crossover upside check
            if self.ma_fast[1] < self.ma_slow[1] && self.ma_fast[0] > self.ma_slow[0] {
                log::info!("[EXIT] Crossover UP");
                let _order_id = self.place_related_order(broker, OrderSide::Sell, OrderType::Market, 1.0, cur_trade_id.unwrap());
            }
        }
    }
}
