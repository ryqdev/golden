pub mod hold;

use crate::green::broker::Broker;

pub trait Strategy {
    fn next(&mut self, _: &Vec<f64>);
}