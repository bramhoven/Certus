use crate::core::{Instrument, Order, Trade};

pub struct Account {
    pub id: String,
    pub balance: f64,
}

pub trait Broker {
    fn place_order(&mut self, order: Order) -> &Order;

    fn add_instrument(&mut self, instrument: Instrument) -> &Instrument;

    fn get_current_position(&mut self, strategy_id: usize, instrument_id: u32) -> f64;

    fn get_open_trades(&mut self, strategy_id: usize, instrument_id: u32) -> Vec<&Trade>;
}
