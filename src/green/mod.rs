pub mod strategy;
pub mod feeds;
pub mod broker;
mod analyzer;
mod visualization;

use std::default::Default;
use crate::err::Error;
use crate::green::{
    feeds::BaseData,
    strategy::Strategy,
    broker::Broker,
    broker::backtest::BackTestBroker,
    analyzer::Analyzer
};
use crate::green::feeds::yahoo::YahooFinanceData;
use crate::strategy::hold::BuyAndHold;


pub struct Green {
    data: Box<YahooFinanceData>,
    strategy: BuyAndHold,
}

pub struct GreenBuilder {
    data: Box<YahooFinanceData>,
    strategy: BuyAndHold,
}

impl Green {
    pub fn new() -> GreenBuilder {
        GreenBuilder{
            ..Default::default()
        }
    }
    pub fn run(&self) {
        log::info!("running...")
    }
    pub fn plot(&self) {
        log::info!("Ploting...");

        // with egui
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "candlestic chart",
            native_options,
            Box::new(|cc| Box::new(visualization::candle::App::default())),
        ).expect("Plotting error");
    }
}


impl GreenBuilder{
    pub fn add_data_feed(&mut self, data: YahooFinanceData) -> &mut GreenBuilder{
        self.data = Box::from(data);
        self
    }
    // pub fn add_broker(&mut self,broker: BackTestBroker) -> &mut GreenBuilder {
    //     self.broker = broker;
    // }
    pub fn add_strategy(&mut self, strategy: BuyAndHold) -> &mut GreenBuilder{
        self.strategy = strategy;
        self
    }
    pub fn add_analyzer(&self, analyzer: Box<dyn Analyzer>) -> &GreenBuilder {
        self
    }
    pub fn build(self) -> Green {
        Green{
            data: self.data,
            strategy: self.strategy,
        }
    }
}