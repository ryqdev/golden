pub trait Broker{
    fn set_cash(&mut self, cash: f64);
}

mod alpaca;
pub(crate) mod ibkr;
pub mod backtest;