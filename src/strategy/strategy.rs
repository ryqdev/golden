use crate::broker::backtest::backtest::{
    Action,
    Order
};
use crate::feeds::Bar;

#[derive(Debug,Default)]
pub struct BaseStrategy{}

impl BaseStrategy {
    pub fn next(&self, data: &Bar) -> Order {
        let threshold = 0.02;
        if data.close > (1.0 + threshold * 3.0) * data.open {
            Order{
                action: Action::Buy,
                size: 1000.0 // unused
            }
        } else if data.close < (1.0 - threshold * 2.0) * data.open{
            Order{
                action: Action::Sell,
                size: 1000.0
            }
        } else {
            Order{
                action: Action::None,
                size: 0.0,
            }
        }
    }

}
