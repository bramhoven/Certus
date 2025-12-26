use certus_core::core::{Trade};

#[test]
fn test_trade_pnl_open_trade() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: 100.0,
        entry_price: 10.0,
        entry_index: 0,
        exit_price: None,
        exit_index: None,
    };
    assert_eq!(trade.pnl(), None);
}

#[test]
fn test_trade_pnl_long_profit() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: 100.0,
        entry_price: 10.0,
        entry_index: 0,
        exit_price: Some(12.0),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(200.0)); // 100 * (12 - 10)
}

#[test]
fn test_trade_pnl_long_loss() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: 100.0,
        entry_price: 10.0,
        entry_index: 0,
        exit_price: Some(8.0),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(-200.0)); // 100 * (8 - 10)
}

#[test]
fn test_trade_pnl_short_profit() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: -100.0,
        entry_price: 10.0,
        entry_index: 0,
        exit_price: Some(8.0),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(200.0)); // -100 * (8 - 10)
}

#[test]
fn test_trade_pnl_short_loss() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: -100.0,
        entry_price: 10.0,
        entry_index: 0,
        exit_price: Some(12.0),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(-200.0)); // -100 * (12 - 10)
}

#[test]
fn test_trade_pnl_break_even() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: 50.0,
        entry_price: 15.0,
        entry_index: 0,
        exit_price: Some(15.0),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(0.0)); // 50 * (15 - 15)
}

#[test]
fn test_trade_pnl_long_fractional() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: 10.5,
        entry_price: 100.25,
        entry_index: 0,
        exit_price: Some(101.50),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(10.5 * (101.50 - 100.25))); // 10.5 * 1.25 = 13.125
}

#[test]
fn test_trade_pnl_short_fractional() {
    let trade = Trade {
        id: 1,
        instrument: 1,
        size: -10.5,
        entry_price: 101.50,
        entry_index: 0,
        exit_price: Some(100.25),
        exit_index: Some(1),
    };
    assert_eq!(trade.pnl(), Some(-10.5 * (100.25 - 101.50))); // -10.5 * -1.25 = 13.125
}