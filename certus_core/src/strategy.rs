use std::collections::VecDeque;

use crate::{core::Order, data::MarketData};

/// trait for trading strategies
/// init and next methods should be implemented
pub trait Strategy {
    /// Initializes a strategy and allows for setting indicators up
    fn init(&mut self);

    /// Will be called before next() is called
    /// Should be used to update indicators
    fn update(&mut self, market_data: MarketData);

    /// Will be called for every new bar or tick, when tick data is supplied
    /// returns a list of orders to be placed
    fn next(&mut self, market_data: MarketData) -> Vec<Order>;
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
            market_data: VecDeque::new()
        }
    }

    pub fn is_ready(&self) -> bool {
        self.market_data.len() == self.max_data_back
    }
}

impl Strategy for StrategyData {
    fn init(&mut self) {}

    fn update(&mut self, market_data: MarketData) {
        self.market_data.push_front(market_data);
        if self.market_data.len() > self.max_data_back {
            self.market_data.truncate(self.max_data_back);
        }
    }

    fn next(&mut self, _market_data: MarketData) -> Vec<Order> {
        Vec::new()
    }
}