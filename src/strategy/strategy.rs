use crate::broker::backtest::backtest::{
    Action,
    Order
};
use crate::feeds::Bar;

#[derive(Debug,Default)]
pub struct BaseStrategy{
    pub(crate) name: String,
}

impl BaseStrategy {
    pub fn next(&self, data: &Bar) -> Order {
        if data.close > data.open {
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
