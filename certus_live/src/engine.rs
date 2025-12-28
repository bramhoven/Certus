use std::hint;

use certus_core::data::DataHandler;
use certus_core::engine::Engine;

pub struct LiveEngine {
    pub data_handler: Box<dyn DataHandler>,
}

impl Engine for LiveEngine {
    fn init(&mut self, data_handler: Box<dyn DataHandler>) {
        self.data_handler = data_handler;
    }

    fn run(&mut self) {
        let mut data_feed = self.data_handler.get_data_feed();
        loop {
            if let Some(market_data) = data_feed.poll() {
                println!("{}", market_data);
            } else {
                hint::spin_loop();
            }
        }
    }
}
