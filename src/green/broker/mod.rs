pub trait Broker{
    fn set_cash(&mut self, cash: f64);
}

pub mod paper;
pub mod live;
pub mod backtest;