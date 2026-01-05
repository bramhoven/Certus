use std::{collections::HashMap, fmt};

/// enum defining the type of trading instrument
#[derive(Debug, Clone)]
pub enum InstrumentType {
    Stock,
    ContinuousFutures {
        big_point_value: f64,
    },
    Futures {
        expiry: String,
        big_point_value: f64,
    },
}

/// struct defining a trading instrument
#[derive(Debug, Clone)]
pub struct Instrument {
    pub id: Option<u32>,
    pub symbol: String,
    pub exchange: Option<String>,
    pub instrument_type: InstrumentType,
}

impl Instrument {
    pub fn new(symbol: String, exchange: Option<String>, instrument_type: InstrumentType) -> Self {
        Self {
            id: None,
            symbol,
            exchange,
            instrument_type,
        }
    }
}

/// enum defining possible order sides
#[derive(Clone, Debug)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// enum defining possible order types
#[derive(Clone, Debug)]
pub enum OrderType {
    Market,
    Limit(f64),
    Stop(f64),
    StopLimit(f64, f64),
}

/// struct for defining an order
#[derive(Clone, Debug)]
pub struct Order {
    pub id: Option<usize>,
    pub related_id: Option<usize>,
    pub instrument: u32,
    pub strategy_id: usize,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: f64,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.related_id.is_none() {
            write!(
                f,
                "Order {} for instrument {}: {:?} {} ({:?})",
                self.id.unwrap_or(0),
                self.instrument,
                self.side,
                self.size,
                self.order_type
            )
        } else {
            write!(
                f,
                "Order {} [related to {}] for instrument {}: {:?} {} ({:?})",
                self.id.unwrap_or(0),
                self.related_id.unwrap_or(0),
                self.instrument,
                self.side,
                self.size,
                self.order_type
            )
        }
    }
}

// struct for defining a fill
#[derive(Clone)]
pub struct Fill {
    pub id: usize,
    pub instrument: u32,
    pub strategy_id: usize,
    pub order_id: usize,
    pub side: OrderSide,
    pub size: f64,
    pub price: f64,
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Fill {} for instrument {} and order {}: {:?} {} ({:?})",
            self.id, self.instrument, self.order_id, self.side, self.size, self.price
        )
    }
}

/// struct for defining a trade
#[derive(Clone)]
pub struct Trade {
    pub id: usize,
    pub instrument: u32,
    pub strategy_id: usize,
    pub fills: Vec<usize>,
    pub size: f64,
    pub entry_price: f64,
    pub entry_index: usize,
    pub exit_price: Option<f64>,
    pub exit_index: Option<usize>,
}

impl Trade {
    /// Calculate the PnL of the trade.
    /// Returns None if the trade is still open (no exit price yet).
    pub fn pnl(&self, big_point_value: f64) -> Option<f64> {
        match self.exit_price {
            Some(exit) => {
                // PnL = size * (exit - entry)
                Some((self.size * (exit - self.entry_price)) * big_point_value)
            }
            None => None, // Trade still open
        }
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Trade {} for instrument {}: size {}, entry at {} (index {}), exit at {:?} (index {:?})",
            self.id,
            self.instrument,
            self.size,
            self.entry_price,
            self.entry_index,
            self.exit_price,
            self.exit_index
        )
    }
}

pub struct PositionManager {
    pub trades: HashMap<usize, Vec<usize>>,
    pub open_trades: HashMap<usize, Vec<usize>>,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            trades: HashMap::new(),
            open_trades: HashMap::new(),
        }
    }

    pub fn get_trades(&self, strategy_id: usize) -> &[usize] {
        self.trades.get(&strategy_id).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn get_open_trades(&self, strategy_id: usize) -> &[usize] {
        self.open_trades.get(&strategy_id).map(Vec::as_slice).unwrap_or(&[])
    }
}
