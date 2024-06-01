pub trait Broker{
    fn set_cash(&mut self, cash: f64);
}

pub(crate) mod alpaca;
pub(crate) mod ibkr;
pub mod backtest;