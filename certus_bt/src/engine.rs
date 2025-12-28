use certus_core::broker::Broker;
use certus_core::core::Order;
use certus_core::data::DataHandler;
use certus_core::engine::{Engine, ExecutionEngine};
use certus_core::strategies::Strategy;

pub struct BacktestingEngine {
    pub data_handler: Box<dyn DataHandler>,
    pub broker: Box<dyn Broker>,
    pub execution_engine: Box<dyn ExecutionEngine>,
    pub strategies: Vec<Box<dyn Strategy>>,
}

impl Engine for BacktestingEngine {
    fn init(&mut self, data_handler: Box<dyn DataHandler>) {
        self.data_handler = data_handler;
    }

    fn run(&mut self) {
        println!("Starting example!");

        println!("Starting data handler!");
        let _ = self.data_handler.start();
        println!("Loaded data!");

        let mut data_feed = self.data_handler.get_data_feed();

        println!("Start polling data feed!");
        while let Some(_market_data) = data_feed.poll() {
            // println!("{}", market_data)
            // if let Some(orders) = strategy.next(market_data) {
            //     if let Some(fills) = self.execution_engine.execute(orders) {
            //         strategy.on_fill(fill);
            //     }
            // }
        }
        println!("Finished data feed!");
    }
}

pub struct BacktestingExecutionEngine {}

impl ExecutionEngine for BacktestingExecutionEngine {
    fn execute_order(&mut self, _order: &mut Order) {}
}
