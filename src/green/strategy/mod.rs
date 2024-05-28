pub mod hold;

use crate::green::broker::Broker;
use crate::green::green::Order;

pub trait Strategy {
    fn next(&mut self, _: &Vec<f64>) -> Order;
}