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
