use std::{collections::VecDeque, ops::Index};

use crate::data::MarketData;

pub trait Indicator {
    fn is_ready(&self) -> bool;
    fn update(&mut self, market_data: MarketData);
}

pub struct MovingAverage {
    pub period: usize,
    pub value: f64,
    window: VecDeque<f64>,
    history_period: usize,
    history: VecDeque<f64>,
}

impl MovingAverage {
    pub fn new(period: usize, history: usize) -> Self {
        Self {
            period,
            value: 0.0,
            window: VecDeque::with_capacity(period),
            history_period: history,
            history: VecDeque::with_capacity(history),
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

        // First pop_back() if length is already at capacity
        // this ensures no new buffer is allocated
        if self.window.len() == self.period {
            self.window.pop_back();
        }
        self.window.push_front(current_price);

        self.value = self.window.iter().sum::<f64>() / self.period as f64;

        // First pop_back() if length is already at capacity
        // this ensures no new buffer is allocated
        if self.history.len() == self.history_period {
            self.history.pop_back();
        }
        self.history.push_front(self.value);
    }
}

impl Index<usize> for MovingAverage {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.history[index]
    }
}
