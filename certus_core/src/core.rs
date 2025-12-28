use std::fmt;

/// enum defining the type of trading instrument
#[derive(Debug, Clone)]
pub enum InstrumentType {
    Stock,
    Futures {
        expiry: String,
        big_point_value: f64,
    },
}

/// struct defining a trading instrument
#[derive(Debug, Clone)]
pub struct Instrument {
    pub symbol: String,
    pub exchange: Option<String>,
    pub instrument_type: InstrumentType,
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
    pub id: usize,
    pub instrument: u32,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub size: f64,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order {} for instrument {}: {:?} {} ({:?})",
            self.id, self.instrument, self.side, self.size, self.order_type
        )
    }
}

/// struct for defining a trade
#[derive(Clone)]
pub struct Trade {
    pub id: usize,
    pub instrument: u32,
    pub size: f64,
    pub entry_price: f64,
    pub entry_index: usize,
    pub exit_price: Option<f64>,
    pub exit_index: Option<usize>,
}

impl Trade {
    /// Calculate the PnL of the trade.
    /// Returns None if the trade is still open (no exit price yet).
    pub fn pnl(&self) -> Option<f64> {
        match self.exit_price {
            Some(exit) => {
                // PnL = size * (exit - entry)
                Some(self.size * (exit - self.entry_price))
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
