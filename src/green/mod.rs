pub mod strategy;
pub mod feeds;
mod broker;
mod analyzer;
mod visualization;

use crate::err::Error;
use crate::green::{
    feeds::BaseData,
    strategy::Strategy,
    broker::Broker,
    analyzer::Analyzer
};

pub struct Green {
    data: String,
}

impl Green {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Green{
            data: "foobar".to_string(),
        })
    }
    pub fn add_data_feed(&self, data: Box<dyn BaseData>){}
    pub fn add_broker(&self,broker: Box<dyn Broker>){}
    pub fn add_strategy(&self, strategy: Box<dyn Strategy>){}
    pub fn add_analyzer(&self, analyzer: Box<dyn Analyzer>){}
    pub fn run(&self) {}
    pub fn plot(&self) {
        log::info!("Ploting...");

        // with egui
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "candlestic chart",
            native_options,
            Box::new(|cc| Box::new(visualization::candle::App::new(cc))),
        ).expect("Plotting error");
    }
}