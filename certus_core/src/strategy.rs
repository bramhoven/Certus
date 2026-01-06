use std::collections::VecDeque;

use crate::{broker::Broker, core::{Order, OrderSide, OrderType}, data::MarketData};

/// trait for trading strategies
/// init and next methods should be implemented
pub trait MarketDataReceiver {
    /// Will be called before next() is called
    /// Should be used to update indicators
    fn update(&mut self, market_data: MarketData, broker: &mut dyn Broker);
}

pub trait StrategyBase : MarketDataReceiver {
    /// Initializes a strategy and allows for setting indicators up
    fn init(&mut self, id: usize);

    /// Get strategy id
    fn get_id(&self) -> usize;

    // get instrument id
    fn get_instrument(&self) -> u32;
}

pub trait Strategy : MarketDataReceiver + StrategyBase {
    /// Will be called for every new bar or tick, when tick data is supplied
    fn next(&mut self, market_data: MarketData, broker: &mut dyn Broker);

    /// Easiest method to place an order
    fn place_order(&mut self, broker: &mut dyn Broker, side: OrderSide, order_type: OrderType, size: f64) -> usize {
        broker.place_order(Order {
            id: None,
            related_id: None,
            instrument: self.get_instrument(),
            strategy_id: self.get_id(),
            side: side,
            order_type: order_type,
            size: size,
        }).id.unwrap()
    }

    /// Easiest method to place a related order
    fn place_related_order(&mut self, broker: &mut dyn Broker, side: OrderSide, order_type: OrderType, size: f64, related_id: usize) -> usize {
        broker.place_order(Order {
            id: None,
            related_id: Some(related_id),
            instrument: self.get_instrument(),
            strategy_id: self.get_id(),
            side: side,
            order_type: order_type,
            size: size,
        }).id.unwrap()
    }
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
            market_data: VecDeque::with_capacity(max_data_back),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.market_data.len() == self.max_data_back
    }
}

impl MarketDataReceiver for StrategyData {
    fn update(&mut self, market_data: MarketData, _broker: &mut dyn Broker) {
        self.market_data.push_front(market_data);
        if self.market_data.len() > self.max_data_back {
            self.market_data.truncate(self.max_data_back);
        }
    }
}
