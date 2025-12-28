pub mod data;
pub mod strategy;

use certus_bt::broker::BacktestingBroker;

use certus_bt::csv_data_handler::CSVDataHandler;
use certus_bt::engine::{BacktestingEngine, BacktestingExecutionEngine};
use certus_core::engine::Engine;

use crate::data::TradeStationCSVRowParser;
use crate::strategy::SimpleStrategy;

fn main() {
    let ts_row_parser = TradeStationCSVRowParser::new();
    let data_handler = CSVDataHandler::new(
        String::from("./data/ES-1M-20150101-20251219.csv"),
        Box::new(ts_row_parser),
    );
    let strategy = SimpleStrategy::new();
    let execution_engine = BacktestingExecutionEngine {};
    let broker = BacktestingBroker::new(100_000.0);

    let mut engine = BacktestingEngine {
        data_handler: Box::new(data_handler),
        broker: Box::new(broker),
        execution_engine: Box::new(execution_engine),
        strategies: vec![Box::new(strategy)],
    };

    engine.run();
}
