use std::{collections::HashMap, mem};

use certus_core::core::{Instrument, OrderType::*};
use certus_core::{
    broker::{Account, Broker},
    core::{Fill, Order, OrderSide, OrderType, Trade},
    data::MarketData,
};

pub struct BacktestingBroker {
    _account: Account,
    orders: HashMap<usize, Order>,
    unfilled_orders: Vec<usize>,
    last_order_id: usize,
    fills: HashMap<usize, Fill>,
    last_fill_id: usize,
    order_trades: HashMap<usize, usize>,
    trades: HashMap<usize, Trade>,
    last_trade_id: usize,
    trade_metrics: HashMap<usize, TradeMetrics>,
    last_instrument_id: u32,
    instruments: HashMap<u32, Instrument>,
}

struct PendingFill {
    order_id: usize,
    stored_order_id: usize,
    fill_size: f64,
    order_remaining: f64,
    instrument: u32,
    strategy_id: usize,
    side: OrderSide,
    order_type: OrderType,
}

#[derive(Default)]
struct TradeMetrics {
    total_size: f64,
    weighted_sum: f64,
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
            order_trades: HashMap::new(),
            trades: HashMap::new(),
            last_trade_id: 0,
            trade_metrics: HashMap::new(),
            last_instrument_id: 0,
            instruments: HashMap::new(),
        }
    }

    pub fn simulate_fills(&mut self, market_data: MarketData) {
        let (mut available_size, current_price) = Self::extract_liquidity(&market_data);

        if available_size <= 0.0 {
            return;
        }

        let order_queue = mem::take(&mut self.unfilled_orders);
        let mut remaining_orders = Vec::new();

        for order_id in order_queue {
            if available_size <= 0.0 {
                remaining_orders.push(order_id);
                continue;
            }

            let Some(pending_fill) = self.prepare_order_fill(order_id, &mut available_size) else {
                continue;
            };

            let price =
                Self::price_for_fill(&pending_fill.order_type, &pending_fill.side, current_price);
            self.record_fill(&pending_fill, price);

            if pending_fill.order_remaining > 0.0 {
                remaining_orders.push(order_id);
            }
        }

        self.unfilled_orders = remaining_orders;
    }

    pub fn get_trade_for_order(&self, order_id: usize) -> Option<&Trade> {
        self.order_trades
            .get(&order_id)
            .and_then(|trade_id| self.trades.get(trade_id))
    }

    pub fn get_fill(&self, fill_id: usize) -> Option<&Fill> {
        self.fills.get(&fill_id)
    }

    pub fn unfilled_orders_len(&self) -> usize {
        self.unfilled_orders.len()
    }

    fn extract_liquidity(market_data: &MarketData) -> (f64, f64) {
        let available_size = match market_data {
            MarketData::Bar(bar) => bar.volume,
            MarketData::Tick(tick) => tick.size,
        };

        let current_price = match market_data {
            MarketData::Bar(bar) => bar.open,
            MarketData::Tick(tick) => tick.price,
        };

        (available_size, current_price)
    }

    fn prepare_order_fill(
        &mut self,
        order_id: usize,
        available_size: &mut f64,
    ) -> Option<PendingFill> {
        let order = self.orders.get_mut(&order_id)?;
        if order.size <= 0.0 {
            return None;
        }

        let size_for_fill = order.size.min(*available_size);
        if size_for_fill <= 0.0 {
            return None;
        }

        *available_size -= size_for_fill;
        order.size -= size_for_fill;

        Some(PendingFill {
            order_id,
            stored_order_id: order.id?,
            fill_size: size_for_fill,
            order_remaining: order.size,
            instrument: order.instrument,
            strategy_id: order.strategy_id,
            side: order.side.clone(),
            order_type: order.order_type.clone(),
        })
    }

    fn record_fill(&mut self, pending_fill: &PendingFill, price: f64) {
        let fill_id = self.store_fill(pending_fill, price);

        match self.order_trades.get(&pending_fill.order_id).copied() {
            Some(trade_id) if trade_id != 0 => {
                self.append_fill_to_trade(trade_id, fill_id, pending_fill.fill_size, price)
            }
            _ => self.create_trade_from_fill(pending_fill, fill_id, price),
        }
    }

    fn price_for_fill(order_type: &OrderType, side: &OrderSide, current_price: f64) -> f64 {
        match order_type {
            Market => current_price,
            Limit(limit) => match side {
                OrderSide::Buy => current_price.min(*limit),
                OrderSide::Sell => current_price.max(*limit),
            },
            Stop(stop) => *stop,
            StopLimit(stop, _limit) => *stop,
        }
    }

    fn store_fill(&mut self, pending_fill: &PendingFill, price: f64) -> usize {
        let fill_id = self.next_fill_id();
        let fill = Fill {
            id: fill_id,
            instrument: pending_fill.instrument,
            strategy_id: pending_fill.strategy_id,
            order_id: pending_fill.stored_order_id,
            side: pending_fill.side.clone(),
            size: pending_fill.fill_size,
            price,
        };
        log::info!("Order {} filled: {}", pending_fill.stored_order_id, fill);
        self.fills.insert(fill_id, fill);
        fill_id
    }

    fn append_fill_to_trade(
        &mut self,
        trade_id: usize,
        fill_id: usize,
        fill_size: f64,
        price: f64,
    ) {
        log::debug!("Adding fill {} to trade {}", fill_id, trade_id);
        if let Some(trade) = self.trades.get_mut(&trade_id) {
            trade.fills.push(fill_id);
            let metrics = self
                .trade_metrics
                .entry(trade_id)
                .or_insert_with(TradeMetrics::default);
            metrics.total_size += fill_size;
            metrics.weighted_sum += fill_size * price;
            let avg_price = if metrics.total_size > 0.0 {
                metrics.weighted_sum / metrics.total_size
            } else {
                trade.entry_price
            };
            trade.entry_price = avg_price;
            trade.size = metrics.total_size;
            log::debug!(
                "Updated entry price for trade {} to {}",
                trade_id,
                avg_price
            );
        }
    }

    fn create_trade_from_fill(&mut self, pending_fill: &PendingFill, fill_id: usize, price: f64) {
        log::debug!("Creating new trade for order {}", pending_fill.order_id);
        let trade_id = self.next_trade_id();
        let trade = Trade {
            id: trade_id,
            instrument: pending_fill.instrument,
            strategy_id: pending_fill.strategy_id,
            fills: vec![fill_id],
            size: pending_fill.fill_size,
            entry_price: price,
            entry_index: 0,
            exit_price: None,
            exit_index: None,
        };
        self.trades.insert(trade_id, trade);
        self.order_trades.insert(pending_fill.order_id, trade_id);
        self.trade_metrics.insert(
            trade_id,
            TradeMetrics {
                total_size: pending_fill.fill_size,
                weighted_sum: pending_fill.fill_size * price,
            },
        );
    }

    fn next_order_id(&mut self) -> usize {
        self.last_order_id += 1;
        self.last_order_id
    }

    fn next_fill_id(&mut self) -> usize {
        self.last_fill_id += 1;
        self.last_fill_id
    }

    fn next_trade_id(&mut self) -> usize {
        self.last_trade_id += 1;
        self.last_trade_id
    }

    fn next_instrument_id(&mut self) -> u32 {
        self.last_instrument_id += 1;
        self.last_instrument_id
    }
}

impl Broker for BacktestingBroker {
    fn place_order(&mut self, mut order: Order) -> &Order {
        let order_id = self.next_order_id();
        
        order.id = Some(order_id);
        self.orders.insert(order_id, order);
        self.unfilled_orders.push(order_id);

        self.orders.get(&order_id).unwrap()
    }

    fn add_instrument(&mut self, mut instrument: Instrument) -> &Instrument {
        let instrument_id = self.next_instrument_id();

        instrument.id = Some(instrument_id);
        self.instruments.insert(instrument_id, instrument);
        self.instruments.get(&instrument_id).unwrap()
    }
}
