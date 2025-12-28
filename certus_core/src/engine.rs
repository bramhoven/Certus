use crate::{core::Order, data::DataHandler};

pub trait Engine {
    fn init(&mut self, data_handler: Box<dyn DataHandler>);
    fn run(&mut self);
}

pub trait ExecutionEngine {
    fn execute_order(&mut self, order: &mut Order);
}
