// import backtrader as bt
//
//
// class SmaCross(bt.SignalStrategy):
// cash = 1000000
//
// def __init__(self):
// sma1, sma2 = bt.ind.SMA(period=10), bt.ind.SMA(period=30)
// crossover = bt.ind.CrossOver(sma1, sma2)
// self.signal_add(bt.SIGNAL_LONG, crossover)


use crate::green::strategy::Strategy;

#[derive(Default, Clone, Copy, Debug)]
pub struct BuyAndHold;

impl Strategy for BuyAndHold{
    fn next(&self){
        todo!()
    }

    fn buy(&self) {
        todo!()
    }

    fn sell(&self) {
        todo!()
    }
}