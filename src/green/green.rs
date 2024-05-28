use std::collections::hash_set::SymmetricDifference;
use std::collections::VecDeque;
use std::default::Default;
use std::fs::File;
use ibapi::orders::{Action, order_builder, OrderNotification};
use crate::cmds::backtest::BackTestCommand;
use crate::err::Error;
use crate::green::{
    feeds::BaseData,
    strategy::Strategy,
    broker::Broker,
    broker::backtest::BackTestBroker,
    analyzer::Analyzer
};
use crate::green::strategy::hold::{Order, SimpleStrategy};
use crate::green::visualization;


#[derive(Default)]
pub struct Green {
    data: Vec<Vec<f64>>,
    strategy: SimpleStrategy,
    broker: BackTestBroker,
}


#[derive(Default)]
pub struct GreenBuilder {
    data: Vec<Vec<f64>>,
    strategy: SimpleStrategy,
    broker: BackTestBroker
}

impl Green {
    pub fn new() -> GreenBuilder {
        GreenBuilder{
            ..Default::default()
        }
    }
    pub fn run(&mut self) {
        log::info!("Running {:?}...", self.strategy);
        for bar in self.data.iter() {
            let order = self.strategy.next(bar);
            let cash = self.broker.cash.last().unwrap();
            let position = self.broker.position.last().unwrap();
            let price = bar[3];
            // TODO: Action is not the struct I defined
            match order.action {
                Action::Buy => {
                    self.broker.cash.push(cash - order.size * price);
                    self.broker.position.push(position + order.size);
                    self.broker.net_assets.push(self.broker.cash.last().unwrap() + self.broker.position.last().unwrap() * price);
                    self.broker.order.push(order)
                }
                Action::Sell => {
                    self.broker.cash.push(cash + order.size * price);
                    self.broker.position.push(position - order.size);
                    self.broker.net_assets.push(self.broker.cash.last().unwrap() + self.broker.position.last().unwrap() * price);
                    self.broker.order.push(order)
                }
                _ => {
                    todo!()
                }
            }
        }
    }
    pub fn plot(&self) {
        log::info!("Plotting {:?}...", self.strategy);
        let candle_data = self.data.clone();
        let cash_data = self.broker.cash.clone();
        let net_asset_data = self.broker.net_assets.clone();
        let order_data = self.broker.order.clone();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("backtest {}", self.strategy.name),
            native_options,
            Box::new(|cc| Box::new(visualization::candle::App{
                candle_data,
                cash_data,
                net_asset_data,
                order_data
            })),
        ).expect("Plotting error");
    }
}

// TODO: add more types in HistoricalData
type HistoricalData = (String, Vec<f64>);

impl GreenBuilder{
    pub fn add_data_feed(&mut self, symbol: &str) -> &mut GreenBuilder{
        let file = File::open(format!("data/{symbol}.csv")).unwrap();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut finance_data = vec![];
        for result in reader.deserialize() {
            let record: HistoricalData = result.unwrap();
            let low = record.1[2];
            let open = record.1[0];
            let close = record.1[3];
            let high = record.1[1];
            finance_data.push(vec![open, high, low, close])
        }
        self.data = finance_data;
        self
    }
    pub fn add_broker(&mut self, cash: f64) -> &mut GreenBuilder {
        self.broker = BackTestBroker{
            cash: Vec::from([cash]),
            position: Vec::from([0.0]),
            net_assets:  Vec::from([cash]),
            order: vec![],
        };
        self
    }
    pub fn add_strategy(&mut self, strategy: SimpleStrategy) -> &mut GreenBuilder{
        self.strategy = strategy;
        self
    }
    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GreenBuilder {
        self
    }
    pub fn build(&self) -> Box<Green> {
        Box::new(Green{
            data: self.data.clone(),
            strategy: self.strategy.clone(),
            broker: self.broker.clone()
        })
    }
}