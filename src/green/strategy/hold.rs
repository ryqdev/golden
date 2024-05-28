use crate::green::green::{Action, Order};
use crate::green::strategy::Strategy;


#[derive(Default, Clone, Debug)]
pub struct SimpleStrategy {}

impl Strategy for SimpleStrategy {
    fn next(&mut self, data: &Vec<f64>) -> Order {
        let open_price = data[0];
        let close_price = data[3];
        if close_price > open_price {
            Order{
                action: Action::Buy,
                size: 1.0
            }
        } else {
            Order{
                action: Action::Sell,
                size: 1.0
            }
        }
    }
}