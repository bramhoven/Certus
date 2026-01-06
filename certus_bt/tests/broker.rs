use certus_bt::broker::BacktestingBroker;
use certus_core::broker::Broker;
use certus_core::core::{Order, OrderSide, OrderType};
use certus_core::data::{MarketData, Tick};

fn make_market_order(side: OrderSide, size: f64, related_id: Option<usize>) -> Order {
    Order {
        id: None,
        related_id,
        instrument: 1,
        strategy_id: 1,
        side,
        order_type: OrderType::Market,
        size,
    }
}

fn make_limit_order(
    side: OrderSide,
    size: f64,
    limit_price: f64,
    related_id: Option<usize>,
) -> Order {
    Order {
        id: None,
        related_id,
        instrument: 1,
        strategy_id: 1,
        side,
        order_type: OrderType::Limit(limit_price),
        size,
    }
}

fn make_stop_order(
    side: OrderSide,
    size: f64,
    stop_price: f64,
    related_id: Option<usize>,
) -> Order {
    Order {
        id: None,
        related_id,
        instrument: 1,
        strategy_id: 1,
        side,
        order_type: OrderType::Stop(stop_price),
        size,
    }
}

fn make_stop_limit_order(
    side: OrderSide,
    size: f64,
    stop_price: f64,
    limit_price: f64,
    related_id: Option<usize>,
) -> Order {
    Order {
        id: None,
        related_id,
        instrument: 1,
        strategy_id: 1,
        side,
        order_type: OrderType::StopLimit(stop_price, limit_price),
        size,
    }
}

fn make_tick(price: f64, size: f64) -> MarketData {
    MarketData::Tick(Tick {
        timestamp: 0,
        price,
        size,
    })
}

#[test]
fn simulate_fills_creates_trade_with_fill_details() {
    let mut broker = BacktestingBroker::new(1_000.0);
    let order_id = broker
        .place_order(make_market_order(OrderSide::Buy, 10.0, None))
        .id
        .unwrap();

    broker.simulate_fills(make_tick(123.45, 10.0));

    assert_eq!(broker.unfilled_orders_len(), 0);
    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected trade for order");
    assert_eq!(trade.fills.len(), 1);
    assert!(
        (trade.entry_price - 123.45).abs() < 1e-12,
        "expected trade entry price 123.45, got {}",
        trade.entry_price
    );
    assert_eq!(trade.size, 10.0);

    let fill_id = trade.fills[0];
    let fill = broker.get_fill(fill_id).expect("expected fill for trade");
    assert_eq!(fill.order_id, order_id);
    assert!(
        (fill.price - 123.45).abs() < 1e-12,
        "expected fill price 123.45, got {}",
        fill.price
    );
    assert_eq!(fill.size, 10.0);
}

#[test]
fn simulate_multiple_fills_updates_average_entry_price() {
    let mut broker = BacktestingBroker::new(1_000.0);
    let order_id = broker
        .place_order(make_market_order(OrderSide::Buy, 10.0, None))
        .id
        .unwrap();

    broker.simulate_fills(make_tick(100.0, 6.0));

    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected initial trade");
    assert_eq!(trade.fills.len(), 1);
    assert!(
        (trade.entry_price - 100.0).abs() < 1e-12,
        "expected initial entry price 100.0, got {}",
        trade.entry_price
    );
    assert_eq!(trade.size, 6.0);
    assert_eq!(broker.unfilled_orders_len(), 1);

    broker.simulate_fills(make_tick(110.0, 4.0));

    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected trade after second fill");
    assert_eq!(trade.fills.len(), 2);
    assert!(
        (trade.entry_price - 104.0).abs() < 1e-12,
        "expected average entry price 104.0, got {}",
        trade.entry_price
    );
    assert_eq!(trade.size, 10.0);
    assert_eq!(broker.unfilled_orders_len(), 0);

    let mut fill_prices: Vec<f64> = trade
        .fills
        .iter()
        .map(|fill_id| broker.get_fill(*fill_id).unwrap().price)
        .collect();
    fill_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(fill_prices, vec![100.0, 110.0]);

    let mut fill_sizes: Vec<f64> = trade
        .fills
        .iter()
        .map(|fill_id| broker.get_fill(*fill_id).unwrap().size)
        .collect();
    fill_sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(fill_sizes, vec![4.0, 6.0]);
}

#[test]
fn related_orders_scale_and_close_trade() {
    let mut broker = BacktestingBroker::new(1_000.0);
    let entry_order_id = broker
        .place_order(make_market_order(OrderSide::Buy, 5.0, None))
        .id
        .unwrap();
    broker.simulate_fills(make_tick(100.0, 5.0));

    let trade = broker
        .get_trade_for_order(entry_order_id)
        .expect("expected trade after initial entry");
    let trade_id = trade.id;
    assert_eq!(trade.size, 5.0);
    assert!(
        (trade.entry_price - 100.0).abs() < 1e-12,
        "expected entry price 100, got {}",
        trade.entry_price
    );

    let add_order_id = broker
        .place_order(make_market_order(
            OrderSide::Buy,
            3.0,
            Some(trade_id),
        ))
        .id
        .unwrap();
    broker.simulate_fills(make_tick(105.0, 3.0));

    let trade = broker
        .get_trade_for_order(add_order_id)
        .expect("expected trade after scaling in");
    assert_eq!(trade.size, 8.0);
    let expected_entry = (5.0 * 100.0 + 3.0 * 105.0) / 8.0;
    assert!(
        (trade.entry_price - expected_entry).abs() < 1e-12,
        "expected average entry {}, got {}",
        expected_entry,
        trade.entry_price
    );
    assert!(trade.exit_price.is_none());

    let reduce_order_id = broker
        .place_order(make_market_order(
            OrderSide::Sell,
            3.0,
            Some(trade_id),
        ))
        .id
        .unwrap();
    broker.simulate_fills(make_tick(110.0, 3.0));

    let trade = broker
        .get_trade_for_order(reduce_order_id)
        .expect("expected trade after scaling out");
    assert_eq!(trade.size, 5.0);
    assert!(
        (trade.entry_price - expected_entry).abs() < 1e-12,
        "expected entry price to remain {}, got {}",
        expected_entry,
        trade.entry_price
    );
    assert!(trade.exit_price.is_none());

    let close_order_id = broker
        .place_order(make_market_order(
            OrderSide::Sell,
            5.0,
            Some(trade_id),
        ))
        .id
        .unwrap();
    broker.simulate_fills(make_tick(115.0, 5.0));

    let trade = broker
        .get_trade_for_order(close_order_id)
        .expect("expected trade after closing");
    assert_eq!(trade.size, 0.0);
    assert_eq!(trade.exit_price, Some(115.0));
}

#[test]
fn market_orders_fill_on_first_available_tick() {
    let mut broker = BacktestingBroker::new(10_000.0);
    let order_id = broker
        .place_order(make_market_order(OrderSide::Buy, 2.0, None))
        .id
        .unwrap();

    assert_eq!(broker.unfilled_orders_len(), 1);

    broker.simulate_fills(make_tick(250.5, 2.0));

    assert_eq!(broker.unfilled_orders_len(), 0);
    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected market order to fill on first tick");
    assert!(
        (trade.entry_price - 250.5).abs() < 1e-12,
        "expected entry price 250.5, got {}",
        trade.entry_price
    );
    assert_eq!(trade.size, 2.0);
}

#[test]
fn limit_orders_wait_for_price_to_meet_limit() {
    let mut broker = BacktestingBroker::new(10_000.0);
    let limit_price = 99.0;
    let order = make_limit_order(OrderSide::Buy, 5.0, limit_price, None);
    let order_id = broker.place_order(order).id.unwrap();

    broker.simulate_fills(make_tick(101.0, 5.0));
    assert!(
        broker.get_trade_for_order(order_id).is_none(),
        "limit order should not fill above limit price"
    );
    assert_eq!(broker.unfilled_orders_len(), 1);

    broker.simulate_fills(make_tick(98.5, 5.0));
    assert_eq!(broker.unfilled_orders_len(), 0);

    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected limit order to fill once price is at or below limit");
    assert!(
        trade.entry_price <= limit_price + 1e-12,
        "expected entry price <= {}, got {}",
        limit_price,
        trade.entry_price
    );
    assert_eq!(trade.size, 5.0);

    let fill = broker
        .get_fill(trade.fills[0])
        .expect("expected fill record for limit order");
    assert!(
        fill.price <= limit_price + 1e-12,
        "expected fill price <= {}, got {}",
        limit_price,
        fill.price
    );
}

#[test]
fn stop_orders_trigger_after_threshold_is_crossed() {
    let mut broker = BacktestingBroker::new(10_000.0);
    let stop_price = 101.0;
    let order = make_stop_order(OrderSide::Buy, 4.0, stop_price, None);
    let order_id = broker.place_order(order).id.unwrap();

    broker.simulate_fills(make_tick(100.25, 4.0));
    assert!(
        broker.get_trade_for_order(order_id).is_none(),
        "stop order should not fill before stop price is reached"
    );

    broker.simulate_fills(make_tick(101.75, 4.0));
    assert_eq!(broker.unfilled_orders_len(), 0);

    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected stop order to fill once price crosses stop");
    assert!(
        trade.entry_price >= stop_price - 1e-12,
        "expected entry price >= {}, got {}",
        stop_price,
        trade.entry_price
    );
    assert_eq!(trade.size, 4.0);
}

#[test]
fn stop_limit_orders_trigger_then_respect_limit_price() {
    let mut broker = BacktestingBroker::new(10_000.0);
    let stop_price = 101.0;
    let limit_price = 101.5;
    let order = make_stop_limit_order(OrderSide::Buy, 3.0, stop_price, limit_price, None);
    let order_id = broker.place_order(order).id.unwrap();

    broker.simulate_fills(make_tick(100.5, 3.0));
    assert!(
        broker.get_trade_for_order(order_id).is_none(),
        "stop-limit order should not fill before stop is triggered"
    );

    broker.simulate_fills(make_tick(102.0, 3.0));
    assert!(
        broker.get_trade_for_order(order_id).is_none(),
        "stop-limit order should trigger but remain pending while price is beyond limit"
    );

    broker.simulate_fills(make_tick(101.25, 3.0));
    assert_eq!(broker.unfilled_orders_len(), 0);

    let trade = broker
        .get_trade_for_order(order_id)
        .expect("expected stop-limit order to fill once price returns within limit");
    assert!(
        trade.entry_price >= stop_price - 1e-12,
        "expected entry price >= stop {}, got {}",
        stop_price,
        trade.entry_price
    );
    assert!(
        trade.entry_price <= limit_price + 1e-12,
        "expected entry price <= limit {}, got {}",
        limit_price,
        trade.entry_price
    );
    assert_eq!(trade.size, 3.0);
}
