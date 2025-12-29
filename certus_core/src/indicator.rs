use std::collections::VecDeque;

use crate::data::MarketData;


pub trait Indicator {
    fn is_ready(&self) -> bool;
    fn update(&mut self, market_data: MarketData);
}

pub struct MovingAverage {
    pub period: usize,
    pub value: f64,
    window: VecDeque<f64>,
}

impl MovingAverage {
    pub fn new(period: usize) -> Self {
        Self {
            period,
            value: 0.0,
            window: VecDeque::new()
        }
    }
}

impl Indicator for MovingAverage {
    fn is_ready(&self) -> bool {
        self.window.len() == self.period
    }

    fn update(&mut self, market_data: MarketData) {
        let current_price = match market_data {
            MarketData::Bar(bar) => bar.close,
            MarketData::Tick(tick) => tick.price,
        };

        if self.window.len() < self.period {
            self.window.push_front(current_price);
        } else {
            self.window.push_front(current_price);
            if self.window.len() > self.period {
                self.window.truncate(self.period);
            }
        }

        self.value = self.window.iter().sum::<f64>() / self.period as f64;
    }
}