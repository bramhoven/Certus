use std::error::Error;

use certus_core::data::{DataFeed, DataHandler, DataHandlerError, MarketData};
use csv::{ReaderBuilder, StringRecord};

pub struct BacktestingDataFeed<'a> {
    index: usize,
    data: &'a [MarketData],
}

impl<'a> DataFeed for BacktestingDataFeed<'a> {
    fn poll(&mut self) -> Option<MarketData> {
        if self.data.len() == 0 || self.index >= self.data.len() {
            return None;
        }

        let cur_data = self.data[self.index];

        self.index += 1;

        Some(cur_data)
    }
}

pub trait CSVRowParser {
    fn parse_row(&mut self, row: StringRecord) -> Result<MarketData, Box<dyn Error>>;
}

pub struct CSVDataHandler {
    pub file_path: String,
    csv_row_parser: Box<dyn CSVRowParser>,
    pub data: Vec<MarketData>,
}

impl CSVDataHandler {
    pub fn new(file_path: String, csv_row_parser: Box<dyn CSVRowParser>) -> Self {
        Self {
            file_path,
            csv_row_parser,
            data: Vec::new(),
        }
    }

    fn load_data(&mut self) -> Result<Vec<MarketData>, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&self.file_path)?;

        let mut data: Vec<MarketData> = Vec::new();

        for result in reader.records() {
            let record = result?;
            data.push(self.csv_row_parser.parse_row(record)?);
        }

        Ok(data)
    }
}

impl DataHandler for CSVDataHandler {
    fn start(&mut self) -> Result<(), DataHandlerError> {
        self.data = match self.load_data() {
            Ok(d) => d,
            Err(e) => {
                print!("{}", e);
                return Err(DataHandlerError::FailedToStart);
            }
        };

        Ok(())
    }

    fn stop(&mut self) {
        self.data = Vec::new();
    }

    fn get_data_feed(&mut self) -> Box<dyn DataFeed + '_> {
        Box::new(BacktestingDataFeed {
            index: 0,
            data: &self.data,
        })
    }
}
