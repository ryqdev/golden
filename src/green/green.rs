use std::fs::File;
use ibapi::{Client};
use ibapi::contracts::{Contract, SecurityType};
use ibapi::market_data::realtime::{BarSize, WhatToShow};

use time::OffsetDateTime;

use crate::green::{
    strategy::{
        Strategy,
        hold::{SimpleStrategy}
    },
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

struct BackTestClient {
    pub(crate) cash: Vec<f64>,
    pub position: Vec<f64>,
    pub(crate) net_assets: Vec<f64>,
    pub order: Vec<Order>
}

struct BaseBroker{
    name: String,
    client: Option<Client>,
    backtest_client: Option<BackTestClient>
}

pub struct Green {
    data: dyn Iterator<Item= Bar>,
    // TODO: change to BaseType
    strategy: SimpleStrategy,
    broker: BaseBroker,
    mode: GreenModeType,
}

pub struct GreenBuilder {
    // use stack or heap, this is a question
    data: dyn Iterator<Item=Bar>,
    strategy: SimpleStrategy,
    broker: BaseBroker,
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
            let order = &self.strategy.next(&bar);
            let mut backtest_client = match &self.broker.backtest_client {
                Some(client) => &client,
                None => todo!()
            };
            let close_price = &bar.close;

            match order.action {
                Action::Buy => {
                    log::info!("Buy: {:?}", order);
                }
                Action::Sell => {
                    log::info!("Sell: {:?}", order);
                }
                _ => todo!()
            }
        }
    }
    pub fn plot(&self) {
        log::info!("Plotting {:?}...", self.strategy);
        let candle_data = &self.data;
        let cash_data = match &self.broker.backtest_client {
            Some(client) => &client.cash,
            None => todo!()
        };
        let net_asset_data = match &self.broker.backtest_client {
            Some(client) => &client.net_assets,
            None => todo!()
        };
        let order_data = match &self.broker.backtest_client {
            Some(client) => &client.order,
            None => todo!()
        };

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
    pub fn set_data_feed(&mut self, symbol: &str) -> &mut GreenBuilder{
        self.data = match self.mode{
            GreenModeType::Backtest => fetch_csv_data(symbol),

            GreenModeType::Paper => self.broker.realtime_bars(&Contract {
                symbol: "USD".to_owned(),
                security_type: SecurityType::ForexPair,
                currency: "JPY".to_owned(),
                exchange: "IDEALPRO".to_owned(),
                ..Default::default()
            }, BarSize::Sec5, WhatToShow::MidPoint, false).unwrap(),

            GreenModeType::Live => todo!()
        };
        self
    }
    pub fn set_mode(&mut self, mode: GreenModeType) -> &mut GreenBuilder{
        self.mode = mode;
        self
    }
    pub fn set_broker(&mut self, cash: f64) -> &mut GreenBuilder {
        self.broker = match self.mode {
            GreenModeType::Backtest => BaseBroker{
                name: "backtest".to_owned(),
                client: None,
                backtest_client: Option::from(BackTestClient {
                    cash: Vec::from([cash]),
                    position: Vec::from([0.0]),
                    net_assets: Vec::from([cash]),
                    order: vec![],
                })

            },
            GreenModeType::Paper => BaseBroker {
                name: "paper".to_owned(),
                client: Some(Client::connect("127.0.0.1:7497", 100).unwrap()),
                backtest_client: None
            },
            GreenModeType::Live => todo!()
        };
        self
    }
    pub fn set_strategy(&mut self, strategy: SimpleStrategy) -> &mut GreenBuilder{
        self.strategy = strategy;
        self
    }
    // pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GreenBuilder {
    //     self
    // }
    pub fn build(&self) -> Box<Green> {
        Box::new(Green{
            data: &self.data,
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