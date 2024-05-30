use std::collections::hash_set::SymmetricDifference;
use std::collections::VecDeque;
use std::fs::File;
use clickhouse::serde::time::datetime64::millis::option::deserialize;
use egui::style::default_text_styles;
use ibapi::{Client, Error};
use ibapi::contracts::{Contract, SecurityType};
use ibapi::market_data::realtime::{BarSize, WhatToShow};

use ibapi::orders::{order_builder, OrderNotification};
use time::OffsetDateTime;

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

#[derive(Debug, Default, Copy, Clone)]
pub enum GreenModeType {
    #[default]
    Backtest,
    Paper,
    Live
}


#[derive(Default, Clone, Debug)]
pub(crate) struct Order {
    pub action: Action,
    pub(crate) size: f64,
}


#[derive(Default)]
pub struct Green {
    // TODO: why to use dyn here
    // TODO: why to use Box here
    data: Box<dyn Iterator<Item = Bar>>,
    // TODO: change to BaseType
    strategy: SimpleStrategy,
    broker: BackTestBroker,
    mode: GreenModeType,
}


#[derive(Default)]
pub struct GreenBuilder {
    data: Box<dyn Iterator<Item = Bar>>,
    strategy: SimpleStrategy,
    broker: BackTestBroker,
    mode: GreenModeType,
}

impl Green {
    pub fn new() -> GreenBuilder {
        GreenBuilder{
            ..Default::default()
        }
    }
    pub fn run(&mut self) {
        log::info!("Running {:?}...", self.strategy);

        // .iter()     : borrows the ownership
        // .into_iter(): transfers the ownership
        for bar in self.data {
            let order = self.strategy.next(&bar);
            let cash = self.broker.cash.last().unwrap();
            let position = self.broker.position.last().unwrap();
            let close_price = &bar.close;
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


impl GreenBuilder {
    pub fn add_data_feed(&mut self, symbol: &str) -> &mut GreenBuilder{
        // TODO: refactor code
        let contract = Contract {
            symbol: "USD".to_owned(),
            security_type: SecurityType::ForexPair,
            currency: "JPY".to_owned(),
            exchange: "IDEALPRO".to_owned(),
            ..Default::default()
        };

        self.data = match self.mode{
            GreenModeType::Backtest => fetch_csv_data(symbol),
            GreenModeType::Paper => self.broker.realtime_bars(&contract, BarSize::Sec5, WhatToShow::MidPoint, false).unwrap(),
            GreenModeType::Live => todo!()
        };
        self
    }
    pub fn set_mode(&mut self, mode: GreenModeType) -> &mut GreenBuilder{
        self.mode = mode;
        self
    }
    pub fn add_broker(&mut self, cash: f64) -> &mut GreenBuilder {
        self.broker = match self.mode {
            GreenModeType::Backtest => BackTestBroker{
                cash: Vec::from([cash]),
                position: Vec::from([0.0]),
                net_assets:  Vec::from([cash]),
                order: vec![],
            },
            GreenModeType::Paper => Client::connect("127.0.0.1:7497", 100).unwrap(),
            GreenModeType::Live => todo!()
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
    pub fn build(&self) -> Box<Green> {
        Box::new(Green{
            data: self.data,
            strategy: self.strategy.clone(),
            broker: self.broker.clone(),
            mode: self.mode,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Bar {
    pub date: OffsetDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub wap: f64,
    pub count: i32,
}


fn fetch_csv_data(symbol: &str) -> impl Iterator<Item = Bar> {
    let csv_file = File::open(format!("data/{symbol}.csv")).unwrap();

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(csv_file)
        .deserialize();

    // TODO: correct the iterator
    reader.map(|record:(String, Vec<f64>)| Bar{
        date: OffsetDateTime::now_utc(),
        open: record.1[0],
        high: record.1[1],
        low: record.1[2],
        close: record.1[3],
        volume: 0.0,
        wap: 0.0,
        count: 0,
    })
}

// fn connect_and_fetch_market_data() -> impl Iterator<Item = Bar>{
//
// }