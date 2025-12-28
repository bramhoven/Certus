use crate::core::Order;

pub struct Account {
    pub id: String,
    pub balance: f64,
}

pub trait Broker {
    fn place_orders(&mut self, orders: Vec<Order>) {
        for order in orders {
            self.place_order(order);
        }
    }

    fn place_order(&mut self, _order: Order) {}
}
