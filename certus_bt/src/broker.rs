use std::collections::HashMap;

use certus_core::core::OrderType::*;
use certus_core::{
    broker::{Account, Broker},
    core::{Fill, Order, OrderSide, Trade},
    data::MarketData,
};

pub struct BacktestingBroker {
    _account: Account,
    orders: HashMap<usize, Order>,
    unfilled_orders: Vec<usize>,
    last_order_id: usize,
    fills: HashMap<usize, Fill>,
    last_fill_id: usize,
    _trades: HashMap<usize, Trade>,
    _last_trade_id: usize,
}

impl BacktestingBroker {
    pub fn new(starting_balance: f64) -> Self {
        Self {
            _account: Account {
                id: String::from("BACKTEST"),
                balance: starting_balance,
            },
            orders: HashMap::new(),
            unfilled_orders: Vec::new(),
            last_order_id: 0,
            fills: HashMap::new(),
            last_fill_id: 0,
            _trades: HashMap::new(),
            _last_trade_id: 0,
        }
    }

    pub fn simulate_fills(&mut self, market_data: MarketData) {
        for order_id in self.unfilled_orders.iter() {
            let order = self.orders.get(order_id).unwrap();

            let fill_id = self.last_fill_id + 1;
            self.last_fill_id = fill_id;

            let current_price = match market_data  {
                MarketData::Bar(bar) => bar.open,
                MarketData::Tick(tick) => tick.price,
            };

            let price = match order.order_type {
                Market => current_price,
                Limit(limit) => match order.side {
                    OrderSide::Buy => {
                        if current_price < limit {
                            current_price
                        } else {
                            limit
                        }
                    }
                    OrderSide::Sell => {
                        if current_price > limit {
                            current_price
                        } else {
                            limit
                        }
                    }
                },
                Stop(stop) => stop,
                StopLimit(stop, _limit) => stop,
            };
            let fill = Fill {
                    id: fill_id,
                    instrument: order.instrument,
                    strategy_id: order.strategy_id,
                    order_id: order.id.unwrap(),
                    side: order.side.clone(),
                    size: order.size,
                    price: price,
                };
            log::info!("Order {} filled: {}", order.id.unwrap(), fill);
            self.fills.insert(
                fill_id,
                fill,
            );
        }
        self.unfilled_orders.clear();
    }
}

impl Broker for BacktestingBroker {
    fn place_order(&mut self, mut order: Order) -> &Order {
        let order_id = self.last_order_id + 1;
        self.last_order_id = order_id;

        order.id = Some(order_id);
        self.orders.insert(order_id, order);
        self.unfilled_orders.push(order_id);

        self.orders.get(&order_id).unwrap()
    }
}
