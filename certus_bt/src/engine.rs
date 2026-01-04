use certus_core::broker::Broker;
use certus_core::core::Order;
use certus_core::data::DataHandler;
use certus_core::engine::{Engine, ExecutionEngine};
use certus_core::strategy::Strategy;

use log;

use crate::broker::BacktestingBroker;

pub struct BacktestingEngine {
    pub data_handler: Box<dyn DataHandler>,
    pub broker: BacktestingBroker,
    pub execution_engine: Box<dyn ExecutionEngine>,
    pub strategies: Vec<Box<dyn Strategy>>,
}

impl Engine for BacktestingEngine {
    fn init(&mut self) {
        log::debug!("Initializing strategies");
        let mut strategy_index: usize = 1;
        for strategy in self.strategies.iter_mut() {
            strategy.init(strategy_index);
            strategy_index += 1;
        }
    }

    fn run(&mut self) {
        log::debug!("Start running");

        let _ = self.data_handler.start();
        let mut data_feed = self.data_handler.get_data_feed();

        log::debug!("Start polling data feed");
        while let Some(market_data) = data_feed.poll() {
            log::debug!("{}", market_data);

            log::debug!("Simulating order fills");
            self.broker.simulate_fills(market_data);

            let broker_ref: &mut dyn Broker = &mut self.broker;
            log::debug!("Calling update() on strategies");
            for strategy in self.strategies.iter_mut() {
                strategy.update(market_data, broker_ref);
            }

            log::debug!("Calling next() on strategies");
            for strategy in self.strategies.iter_mut() {
                strategy.next(market_data, broker_ref);
            }
            // if let Some(orders) = strategy.next(market_data) {
            //     if let Some(fills) = self.execution_engine.execute(orders) {
            //         strategy.on_fill(fill);
            //     }
            // }
        }
        log::debug!("Finished data feed");
    }
}

pub struct BacktestingExecutionEngine {}

impl ExecutionEngine for BacktestingExecutionEngine {
    fn execute_order(&mut self, _order: &mut Order) {}
}
