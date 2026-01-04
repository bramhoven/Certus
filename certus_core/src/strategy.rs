use std::collections::VecDeque;

use crate::{broker::Broker, data::MarketData};

/// trait for trading strategies
/// init and next methods should be implemented
pub trait Strategy {
    /// Initializes a strategy and allows for setting indicators up
    fn init(&mut self, id: usize);

    /// Will be called before next() is called
    /// Should be used to update indicators
    fn update(&mut self, market_data: MarketData, broker: &mut dyn Broker);

    /// Will be called for every new bar or tick, when tick data is supplied
    fn next(&mut self, market_data: MarketData, broker: &mut dyn Broker);
}

#[derive(Debug, Default)]
pub struct StrategyData {
    pub max_data_back: usize,
    pub market_data: VecDeque<MarketData>,
}

impl StrategyData {
    pub fn new(max_data_back: usize) -> Self {
        Self {
            max_data_back,
            market_data: VecDeque::with_capacity(max_data_back)
        }
    }

    pub fn is_ready(&self) -> bool {
        self.market_data.len() == self.max_data_back
    }
}

impl Strategy for StrategyData {
    fn init(&mut self, _id: usize) {}

    fn update(&mut self, market_data: MarketData, _broker: &mut dyn Broker) {
        self.market_data.push_front(market_data);
        if self.market_data.len() > self.max_data_back {
            self.market_data.truncate(self.max_data_back);
        }
    }

    fn next(&mut self, _market_data: MarketData, _broker: &mut dyn Broker) {}
}