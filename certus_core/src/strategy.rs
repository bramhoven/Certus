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
