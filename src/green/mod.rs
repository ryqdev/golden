pub mod strategy;
pub mod feeds;
pub mod broker;
mod analyzer;
mod visualization;

use std::default::Default;
use std::fs::File;
use ibapi::orders::{Action, order_builder, OrderNotification};
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
    // TODO: move to Heap?
    data: Vec<Vec<f64>>,
    strategy: BuyAndHold,
    broker: dyn Broker,
}

pub struct GreenBuilder {
    data: Vec<Vec<f64>>,
    strategy: BuyAndHold,
    broker: Box<dyn Broker>,
}

impl Green {
    pub fn new() -> GreenBuilder {
        GreenBuilder{
            ..Default::default()
        }
    }
    pub fn run(&self) {
        log::info!("running...");
        let client = Broker.connect()
        // TODO: multi-thread with self.data and self.strategy?
        for bar in self.data {
            log::info!("\x1b[93m bar:\x1b[0m {:?} ", bar);
            let action = Action::Buy;

            // TODO: build the client
            let order_id = client.next_order_id();
            let order = order_builder::market_order(action, 1000.0);

            let notices = client.place_order(order_id, &contract, &order).unwrap();
            for notice in notices {
                if let OrderNotification::ExecutionData(data) = notice {
                    println!("{} {} shares of {}", data.execution.side, data.execution.shares, data.contract.symbol);
                } else {
                    println!("{:?}", notice);
                }
            }

        }
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

type HistoricalData = (String, Vec<f64>, usize);

impl GreenBuilder{
    pub fn add_data_feed(&mut self, symbol: &str) -> &mut GreenBuilder{
        // Date,Open,High,Low,Close,Adj Close,Volume
        let file = File::open(format!("data/{symbol}.csv"))?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut finance_data = vec![];
        for result in reader.deserialize() {
            let record: HistoricalData = result?;
            let low = record.1[2];
            let open = record.1[0];
            let close = record.1[3];
            let high = record.1[1];
            finance_data.push(vec![open, high, low, close])
        }
        self.data = finance_data;
        self
    }
    pub fn add_broker(&mut self, broker: BackTestBroker) -> &mut GreenBuilder {
        self.broker = Box::new(BackTestBroker {
            cash: broker.cash,
            net_assets: broker.cash
        });
        self
    }
    pub fn add_strategy(&mut self, strategy: BuyAndHold) -> &mut GreenBuilder{
        self.strategy = strategy;
        self
    }
    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GreenBuilder {
        self
    }
    pub fn build(&self) -> Box<Green> {
        Green{
            // TODO: remove clone?
            data: self.data.clone(),
            strategy: self.strategy.clone(),
            broker: self.broker.clone()
        }
    }
}