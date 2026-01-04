use chrono::prelude::*;
use csv::StringRecord;

use certus_bt::csv_data_handler::CSVRowParser;
use certus_core::data::{Bar, MarketData};

pub struct TradeStationCSVRowParser {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}
impl TradeStationCSVRowParser {
    pub fn new() -> Self {
        Self {
            date: String::from(""),
            open: 0.0,
            high: 0.0,
            low: 0.0,
            close: 0.0,
            volume: 0.0,
        }
    }
}
impl CSVRowParser for TradeStationCSVRowParser {
    fn parse_row(
        &mut self,
        row: StringRecord,
    ) -> Result<certus_core::data::MarketData, Box<dyn std::error::Error>> {
        let date_str = row[0].to_string();
        let time_str = row[1].to_string();
        self.date = format!("{date_str} {time_str}"); // 01/02/2015 09:01

        self.open = row[2].parse::<f64>()?;
        self.high = row[3].parse::<f64>()?;
        self.low = row[4].parse::<f64>()?;
        self.close = row[5].parse::<f64>()?;
        self.volume = row[8].parse::<f64>()?;

        Ok(MarketData::Bar(Bar {
            date: NaiveDateTime::parse_from_str(&self.date, "%m/%d/%Y %H:%M").unwrap(),
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
        }))
    }
}
