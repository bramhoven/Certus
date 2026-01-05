use certus_core::core::Trade;

fn make_trade(size: f64, entry_price: f64, exit_price: Option<f64>) -> Trade {
    Trade {
        id: 1,
        instrument: 1,
        strategy_id: 1,
        fills: vec![],
        size,
        entry_price,
        entry_index: 0,
        exit_price,
        exit_index: exit_price.map(|_| 1),
    }
}

const STOCK_BIG_POINT_VALUE: f64 = 1.0;

#[test]
fn test_trade_pnl_open_trade() {
    let trade = make_trade(100.0, 10.0, None);
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), None);
}

#[test]
fn test_trade_pnl_long_profit() {
    let trade = make_trade(100.0, 10.0, Some(12.0));
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(200.0)); // 100 * (12 - 10)
}

#[test]
fn test_trade_pnl_long_loss() {
    let trade = make_trade(100.0, 10.0, Some(8.0));
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(-200.0)); // 100 * (8 - 10)
}

#[test]
fn test_trade_pnl_short_profit() {
    let trade = make_trade(-100.0, 10.0, Some(8.0));
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(200.0)); // -100 * (8 - 10)
}

#[test]
fn test_trade_pnl_short_loss() {
    let trade = make_trade(-100.0, 10.0, Some(12.0));
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(-200.0)); // -100 * (12 - 10)
}

#[test]
fn test_trade_pnl_break_even() {
    let trade = make_trade(50.0, 15.0, Some(15.0));
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(0.0)); // 50 * (15 - 15)
}

#[test]
fn test_trade_pnl_long_fractional() {
    let trade = make_trade(10.5, 100.25, Some(101.50));
    let expected = 10.5 * (101.50 - 100.25);
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(expected)); // 10.5 * 1.25 = 13.125
}

#[test]
fn test_trade_pnl_short_fractional() {
    let trade = make_trade(-10.5, 101.50, Some(100.25));
    let expected = -10.5 * (100.25 - 101.50);
    assert_eq!(trade.pnl(STOCK_BIG_POINT_VALUE), Some(expected)); // -10.5 * -1.25 = 13.125
}

#[test]
fn test_trade_pnl_continuous_futures_big_point() {
    let trade = make_trade(2.0, 4000.0, Some(4001.0));
    let expected = 2.0 * (4001.0 - 4000.0) * 50.0;
    assert_eq!(trade.pnl(50.0), Some(expected));
}

#[test]
fn test_trade_pnl_dated_futures_big_point_loss() {
    let trade = make_trade(-3.0, 200.0, Some(202.0));
    let expected = -3.0 * (202.0 - 200.0) * 12.5;
    assert_eq!(trade.pnl(12.5), Some(expected));
}
