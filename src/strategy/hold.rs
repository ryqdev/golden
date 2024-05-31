use crate::green::{
    green::{Action, Order, Bar},
    strategy::Strategy
};



#[derive(Default, Clone, Debug)]
pub struct SimpleStrategy {}

impl Strategy for SimpleStrategy {
    fn next(&mut self, data: &Bar) -> Order {
        if data.close >  data.open {
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