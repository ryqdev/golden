use std::collections::hash_set::SymmetricDifference;
use std::collections::VecDeque;
use std::default::Default;
use std::fs::File;

use ibapi::orders::{order_builder, OrderNotification};

use crate::cmds::backtest::BackTestCommand;
use crate::green::{
    feeds::BaseData,
    strategy::{
        Strategy,
        hold::{SimpleStrategy}
    },
    broker::Broker,
    broker::backtest::BackTestBroker,
    analyzer::Analyzer,
    visualization
};

#[derive(Debug, Default, Copy, Clone)]
pub enum Action {
    #[default]
    None,
    Buy,
    Sell,
}


#[derive(Default, Clone, Debug)]
pub(crate) struct Order {
    pub action: Action,
    pub(crate) size: f64,
}


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
            let close_price = bar.last().unwrap();
            match order.action {
                Action::Buy => {
                    log::info!("Buy: {:?}", order);
                    self.broker.cash.push(cash - order.size * close_price);
                    self.broker.position.push(position + order.size);
                    self.broker.net_assets.push(self.broker.cash.last().unwrap() + self.broker.position.last().unwrap() * close_price);
                    self.broker.order.push(order)
                }
                Action::Sell => {
                    log::info!("Sell: {:?}", order);
                    self.broker.cash.push(cash + order.size * close_price);
                    self.broker.position.push(position - order.size);
                    self.broker.net_assets.push(self.broker.cash.last().unwrap() + self.broker.position.last().unwrap() * close_price);
                    self.broker.order.push(order)
                }
                _ => ()
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
            &format!("backtest {:?}", self.strategy),
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
    // pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GreenBuilder {
    //     self
    // }
    pub fn init(&self) -> Box<Green> {
        Box::new(Green{
            data: self.data.clone(),
            strategy: self.strategy.clone(),
            broker: self.broker.clone()
        })
    }
}