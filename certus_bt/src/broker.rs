use certus_core::broker::{Account, Broker};

pub struct BacktestingBroker {
    _account: Account,
}

impl BacktestingBroker {
    pub fn new(starting_balance: f64) -> Self {
        Self {
            _account: Account {
                id: String::from("BACKTEST"),
                balance: starting_balance,
            },
        }
    }
}

impl Broker for BacktestingBroker {}
