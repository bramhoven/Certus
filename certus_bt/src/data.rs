use std::collections::HashMap;

use chrono::prelude::*;
use chrono::NaiveDateTime;

use certus_core::data::{Bar, MarketData};

pub struct HistoricBarConsolidationModel {
    pub input_minutes: u32,
    pub output_minutes: u32,
}

impl HistoricBarConsolidationModel {
    pub fn new(input_minutes: u32, output_minutes: u32) -> Self {
        assert!(output_minutes % input_minutes == 0,
            "output_minutes ({}) must be a multiple of input_minutes ({})",
            output_minutes,
            input_minutes);

        Self {
            input_minutes,
            output_minutes,
        }
    }

    fn bucket_start(&self, dt: NaiveDateTime) -> NaiveDateTime {
        let minute = dt.minute() as u32;
        let bucket_minute = (minute / self.output_minutes) * self.output_minutes;

        dt.with_minute(bucket_minute as u32)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }

    pub fn consolidate_bars(&self, data: &Vec<MarketData>) -> Vec<MarketData>  {
        let mut buckets: HashMap<NaiveDateTime, Vec<Bar>> = HashMap::new();

        for single_data in data.iter() {
            let bar = match single_data {
                MarketData::Bar(bar) => bar,
                other => panic!("Expected MarkeData::Bar, got {:?}", other),
            };

            let bucket_start = self.bucket_start(bar.date);
            buckets.entry(bucket_start).or_default().push(*bar);
        }

        let mut result: Vec<MarketData> = Vec::new();

        for (bucket_start, mut bars) in buckets {
            // Sort bars inside the bucket
            bars.sort_by_key(|b| b.date);

            let open = bars.first().unwrap().open;
            let close = bars.last().unwrap().close;

            let high = bars.iter().map(|b| b.high).fold(f64::MIN, f64::max);
            let low = bars.iter().map(|b| b.low).fold(f64::MAX, f64::min);
            let volume = bars.iter().map(|b| b.volume).sum();

            result.push(MarketData::Bar(Bar {
                date: bucket_start,
                open,
                high,
                low,
                close,
                volume,
            }));
        }

        result.sort_by_key(|data| {
            match data {
                MarketData::Bar(bar) => bar.date,
                _ => unreachable!("Expected only MarketData::Bar"),
            }
        });
        result
    }
}