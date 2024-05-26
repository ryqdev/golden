pub mod hold;

use crate::green::broker::backtest::BackTestBroker;
use crate::green::broker::Broker;

pub trait Strategy {
    fn next(&mut self, _: &Vec<f64>);

    fn buy(&mut self, size: f64, price: f64);

    fn sell(&mut self, size: f64, price: f64);

    // fn update_broker(&self, broker: &mut BackTestBroker){
    //     broker.set_cash(self.cash);
    // }

}