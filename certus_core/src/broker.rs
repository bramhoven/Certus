use core::panic;

use crate::core::{Instrument, Order, Trade};

pub struct Account {
    pub id: String,
    pub balance: f64,
}

pub trait Broker {
    fn place_order(&mut self, _order: Order) -> &Order {
        panic!("This method needs to be implemented!")
    }

    fn add_instrument(&mut self, _instrument: Instrument) -> &Instrument {
        panic!("This method needs to be implemented!")
    }

    fn get_current_position(&mut self, _strategy_id: usize, _instrument_id: u32) -> f64 {
        panic!("This method needs to be implemented!")
    }

    fn get_open_trades(&mut self, _strategy_id: usize, _instrument_id: u32) -> Vec<&Trade> {
        panic!("This method needs to be implemented!")
    }
}
