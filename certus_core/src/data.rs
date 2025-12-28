use chrono::prelude::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Tick {
    pub timestamp: i64,
    pub price: f64,
    pub size: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct Bar {
    pub date: NaiveDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Copy, Clone)]
pub enum MarketData {
    Tick(Tick),
    Bar(Bar),
}

impl fmt::Display for MarketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketData::Tick(tick) => write!(
                f,
                "Tick(timestamp: {}, price: {}, size: {})",
                tick.timestamp, tick.price, tick.size
            ),
            MarketData::Bar(bar) => write!(
                f,
                "Bar(date: {}, open: {}, high: {}, low: {}, close: {}, volume: {})",
                bar.date, bar.open, bar.high, bar.low, bar.close, bar.volume
            ),
        }
    }
}

pub trait DataHandler {
    fn start(&mut self) -> Result<(), DataHandlerError>;
    fn stop(&mut self);
    fn get_data_feed(&mut self) -> Box<dyn DataFeed + '_>;
}

pub trait DataFeed {
    fn poll(&mut self) -> Option<MarketData>;
}

#[derive(Debug)]
pub enum DataHandlerError {
    FailedToStart,
}
