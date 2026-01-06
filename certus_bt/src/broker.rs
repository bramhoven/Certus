use std::{collections::HashMap, mem};

use certus_core::core::{Instrument, PositionManager};
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
    position_manager: PositionManager,
}

struct PendingFill {
    order_id: usize,
    stored_order_id: usize,
    related_trade_id: Option<usize>,
    fill_size: f64,
    signed_quantity: f64,
    order_remaining: f64,
    instrument: u32,
    strategy_id: usize,
    side: OrderSide,
    order_type: OrderType,
}

#[derive(Default)]
struct TradeMetrics {
    net_quantity: f64,
    entry_weighted_sum: f64,
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
            position_manager: PositionManager::new(),
        }
    }

    pub fn simulate_fills(&mut self, market_data: MarketData) {
        let mut available_size = Self::extract_liquidity(&market_data);

        if available_size <= 0.0 {
            return;
        }

        let order_queue = mem::take(&mut self.unfilled_orders);
        let mut remaining_orders = Vec::new();

        let prices: Vec<f64> = match market_data {
            MarketData::Tick(tick) => vec![tick.price],
            MarketData::Bar(bar) => vec![bar.open, bar.high, bar.low, bar.close],
        };

        let lowest_price = prices.iter().cloned().reduce(f64::min).unwrap();
        let highest_price = prices.iter().cloned().reduce(f64::min).unwrap();

        for order_id in order_queue {
            if available_size <= 0.0 {
                remaining_orders.push(order_id);
                continue;
            }

            if !self.check_order_hit(order_id, lowest_price, highest_price) {
                remaining_orders.push(order_id);
                continue;
            }

            let Some(pending_fill) = self.prepare_order_fill(order_id, &mut available_size) else {
                remaining_orders.push(order_id);
                continue;
            };

            let price =
                Self::price_for_fill(&pending_fill.order_type, &pending_fill.side, &market_data, lowest_price, highest_price);
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

    fn extract_liquidity(market_data: &MarketData) -> f64 {
        let available_size = match market_data {
            MarketData::Bar(bar) => bar.volume,
            MarketData::Tick(tick) => tick.size,
        };

        available_size
    }

    fn signed_quantity(side: &OrderSide, size: f64) -> f64 {
        match side {
            OrderSide::Buy => size,
            OrderSide::Sell => -size,
        }
    }

    fn ensure_trade_is_open(&mut self, strategy_id: usize, trade_id: usize) {
        let entry = self
            .position_manager
            .open_trades
            .entry(strategy_id)
            .or_insert_with(Vec::new);
        if !entry.iter().any(|id| *id == trade_id) {
            entry.push(trade_id);
        }
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
            related_trade_id: order.related_id,
            fill_size: size_for_fill,
            signed_quantity: Self::signed_quantity(&order.side, size_for_fill),
            order_remaining: order.size,
            instrument: order.instrument,
            strategy_id: order.strategy_id,
            side: order.side.clone(),
            order_type: order.order_type.clone(),
        })
    }

    fn check_order_hit(&self, order_id: usize, lowest_price: f64, highest_price: f64) -> bool {
        let order = self.orders.get(&order_id).unwrap();
        match order.order_type {
            OrderType::Market => true,
            OrderType::Limit(limit) => match order.side {
                OrderSide::Buy => lowest_price <= limit,
                OrderSide::Sell => highest_price >= limit
            },
            OrderType::Stop(stop) => match order.side {
                OrderSide::Buy => highest_price >= stop,
                OrderSide::Sell => lowest_price <= stop,
            }
            OrderType::StopLimit(stop, limit) => match order.side {
                OrderSide::Buy => highest_price >= stop && lowest_price <= limit,
                OrderSide::Sell => lowest_price <= stop && highest_price >= limit,
            }
        }
    }

    fn record_fill(&mut self, pending_fill: &PendingFill, price: f64) {
        let fill_id = self.store_fill(pending_fill, price);

        let related_trade = pending_fill
            .related_trade_id
            .filter(|trade_id| self.trades.contains_key(trade_id));

        let trade_id = related_trade
            .or_else(|| self.order_trades.get(&pending_fill.order_id).copied())
            .unwrap_or(0);

        if trade_id == 0 {
            self.create_trade_from_fill(pending_fill, fill_id, price);
        } else {
            self.order_trades
                .insert(pending_fill.order_id, trade_id);
            self.append_fill_to_trade(trade_id, fill_id, pending_fill, price);
        }
    }

    // Calculate the price for the fill
    // Ensure this also works with bars that gap the limit and stops
    fn price_for_fill(order_type: &OrderType, side: &OrderSide, market_data: &MarketData, lowest_price: f64, highest_price: f64) -> f64 {
        match *order_type {
            OrderType::Market => match market_data {
                MarketData::Tick(tick) => tick.price,
                MarketData::Bar(bar) => bar.open,
            },
            OrderType::Limit(limit) => match side {
                OrderSide::Buy => if highest_price >= limit && lowest_price <= limit { limit } else { limit.max(highest_price) },
                OrderSide::Sell => if highest_price >= limit && lowest_price <= limit { limit } else { limit.min(lowest_price) },
            },
            OrderType::Stop(stop) => match side {
                OrderSide::Buy => if highest_price >= stop && lowest_price <= stop { stop } else { stop.min(lowest_price) },
                OrderSide::Sell => if highest_price >= stop && lowest_price <= stop { stop } else { stop.max(highest_price) },
            },
            OrderType::StopLimit(stop, limit) => match side {
                OrderSide::Buy => if highest_price >= stop && lowest_price <= limit { stop } else { limit.min(lowest_price) },
                OrderSide::Sell => if highest_price >= stop && lowest_price <= limit { stop } else { limit.max(highest_price) },
            },
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
        pending_fill: &PendingFill,
        price: f64,
    ) {
        log::debug!("Adding fill {} to trade {}", fill_id, trade_id);
        let mut ensure_open_strategy: Option<usize> = None;
        let mut remove_from_open_strategy: Option<usize> = None;
        if let Some(trade) = self.trades.get_mut(&trade_id) {
            trade.fills.push(fill_id);
            let metrics = self
                .trade_metrics
                .entry(trade_id)
                .or_insert_with(TradeMetrics::default);
            let signed_quantity = pending_fill.signed_quantity;
            let fill_size = pending_fill.fill_size;

            if metrics.net_quantity.abs() <= f64::EPSILON {
                metrics.net_quantity = signed_quantity;
                metrics.entry_weighted_sum = price * fill_size;
                trade.entry_price = price;
                trade.size = signed_quantity;
                trade.exit_price = None;
                trade.exit_index = None;
                ensure_open_strategy = Some(trade.strategy_id);
            } else {
                let prev_net = metrics.net_quantity;
                let prev_sign = prev_net.signum();
                if prev_sign == signed_quantity.signum() {
                    metrics.net_quantity += signed_quantity;
                    metrics.entry_weighted_sum += price * fill_size;
                    if metrics.net_quantity.abs() > f64::EPSILON {
                        let avg_price =
                            metrics.entry_weighted_sum / metrics.net_quantity.abs();
                        trade.entry_price = avg_price;
                        trade.size = metrics.net_quantity;
                        trade.exit_price = None;
                        trade.exit_index = None;
                        ensure_open_strategy = Some(trade.strategy_id);
                        log::debug!(
                            "Updated entry price for trade {} to {}",
                            trade_id,
                            avg_price
                        );
                    }
                } else {
                    let prev_abs = prev_net.abs();
                    let close_qty = fill_size.min(prev_abs);
                    if prev_abs > f64::EPSILON {
                        let entry_avg = metrics.entry_weighted_sum / prev_abs;
                        metrics.entry_weighted_sum -= entry_avg * close_qty;
                    }
                    metrics.net_quantity = prev_net - prev_sign * close_qty;

                    let remaining = fill_size - close_qty;
                    if remaining > f64::EPSILON {
                        let new_net = signed_quantity.signum() * remaining;
                        metrics.net_quantity = new_net;
                        metrics.entry_weighted_sum = price * remaining;
                        trade.entry_price = price;
                        trade.size = new_net;
                        trade.exit_price = None;
                        trade.exit_index = None;
                        ensure_open_strategy = Some(trade.strategy_id);
                        log::warn!(
                            "Order {} over-closed trade {} by {}, reopening with {} @ {}",
                            pending_fill.order_id,
                            trade_id,
                            remaining,
                            new_net,
                            price
                        );
                    } else if metrics.net_quantity.abs() > f64::EPSILON {
                        let avg_price =
                            metrics.entry_weighted_sum / metrics.net_quantity.abs();
                        trade.entry_price = avg_price;
                        trade.size = metrics.net_quantity;
                        trade.exit_price = None;
                        trade.exit_index = None;
                        ensure_open_strategy = Some(trade.strategy_id);
                    } else {
                        metrics.entry_weighted_sum = 0.0;
                        trade.size = 0.0;
                        trade.exit_price = Some(price);
                        trade.exit_index = Some(trade.fills.len());
                        self.trade_metrics.remove(&trade_id);
                        remove_from_open_strategy = Some(trade.strategy_id);
                        log::debug!("Trade {} closed at {}", trade_id, price);
                    }
                }
            }
        }
        if let Some(strategy_id) = remove_from_open_strategy {
            if let Some(open) = self.position_manager.open_trades.get_mut(&strategy_id) {
                open.retain(|id| *id != trade_id);
            }
        }
        if let Some(strategy_id) = ensure_open_strategy {
            self.ensure_trade_is_open(strategy_id, trade_id);
        }
    }

    fn create_trade_from_fill(&mut self, pending_fill: &PendingFill, fill_id: usize, price: f64) {
        log::debug!("Creating new trade for order {}", pending_fill.order_id);
        let trade_id = self.next_trade_id();
        let signed_quantity = pending_fill.signed_quantity;
        let trade = Trade {
            id: trade_id,
            instrument: pending_fill.instrument,
            strategy_id: pending_fill.strategy_id,
            fills: vec![fill_id],
            size: signed_quantity,
            entry_price: price,
            entry_index: 0,
            exit_price: None,
            exit_index: None,
        };

        // Set position
        self.position_manager
            .trades
            .entry(trade.strategy_id)
            .or_insert_with(Vec::new)
            .push(trade_id);
        self.ensure_trade_is_open(trade.strategy_id, trade_id);

        self.trades.insert(trade_id, trade);
        self.order_trades.insert(pending_fill.order_id, trade_id);
        self.trade_metrics.insert(
            trade_id,
            TradeMetrics {
                net_quantity: signed_quantity,
                entry_weighted_sum: pending_fill.fill_size * price,
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
        if let Some(related_id) = order.related_id {
            if let Some(trade) = self.trades.get(&related_id) {
                if trade.instrument != order.instrument {
                    log::warn!(
                        "Order {} references trade {} with mismatched instrument ({:?} vs {:?})",
                        order_id,
                        related_id,
                        order.instrument,
                        trade.instrument
                    );
                }
            } else {
                log::warn!(
                    "Order {} references missing trade {}",
                    order_id,
                    related_id
                );
            }
        }
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

    fn get_current_position(&mut self, strategy_id: usize, instrument_id: u32) -> f64 {
        self
            .position_manager
            .get_open_trades(strategy_id)
            .iter()
            .filter_map(|k| self.trades.get(k))
            .filter(|t| t.instrument == instrument_id)
            .map(|t| t.size)
            .sum()
    }

    fn get_open_trades(&mut self, strategy_id: usize, instrument_id: u32) -> Vec<&Trade> {
        self
            .position_manager
            .get_open_trades(strategy_id)
            .iter()
            .filter_map(|k| self.trades.get(k))
            .filter(|t| t.instrument == instrument_id)
            .collect()
    }
}
