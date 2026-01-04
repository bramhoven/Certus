use crate::core::Order;

pub trait Engine {
    fn init(&mut self);
    fn run(&mut self);
}

pub trait ExecutionEngine {
    fn execute_order(&mut self, order: &mut Order);
}
