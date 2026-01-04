use core::panic;

use crate::core::{Instrument, Order};

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
}
