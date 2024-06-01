// TODO: doc

pub mod cmds;
mod err;
pub mod feeds;
pub mod broker;
mod analyzer;
mod visualization;
pub mod strategy;

use std::fs::File;
use std::io::Read;
use ibapi::{Client};

use time::OffsetDateTime;
use err::Error;

#[derive(Debug, Default, Copy, Clone)]
pub enum Action {
    #[default]
    None,
    Buy,
    Sell,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum GoldenModeType {
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

#[derive(Default)]
struct BaseBroker{
    name: String,
    client: Option<Client>,
    backtest_client: Option<BackTestClient>
}

#[derive(Debug,Default)]
pub struct BaseStrategy{
    name: String,
}

impl BaseStrategy {
    fn next(bar: &Bar) {
    }
}

pub struct Golden {
    data: Box<dyn Iterator<Item = Bar>>,
    strategy: BaseStrategy,
    broker: BaseBroker,
    mode: GoldenModeType,
}

// TODO: any better implementations?
impl Default for Golden {
    fn default() -> Self {
        Golden {
            data: Box::new(std::iter::empty()),
            ..Default::default()
        }
    }
}


impl Golden {
    pub fn new() -> Golden {
        Golden{
            ..Default::default()
        }
    }
    pub fn run(&self) {
        log::info!("Running {:?}...", self.strategy);

        // .iter()     : borrows the ownership
        // .into_iter(): transfers the ownership
        // for bar in self.data {
            // let order = &self.strategy.next(&bar);
            // let mut backtest_client = match &self.broker.backtest_client {
            //     Some(client) => &client,
            //     None => todo!()
            // };
            // let close_price = &bar.close;

            // match order.action {
            //     Action::Buy => {
            //         log::info!("Buy: {:?}", order);
            //     }
            //     Action::Sell => {
            //         log::info!("Sell: {:?}", order);
            //     }
            //     _ => todo!()
            // }
        // }
    }
    pub fn plot(&self) {
        // lifetime!!!
        log::info!("Plotting {:?}...", self.strategy);
        // let candle_data = &self.data;
        // let cash_data = match &self.broker.backtest_client {
        //     Some(client) => &client.cash,
        //     None => todo!()
        // };
        // let net_asset_data = match &self.broker.backtest_client {
        //     Some(client) => &client.net_assets,
        //     None => todo!()
        // };
        // let order_data = match &self.broker.backtest_client {
        //     Some(client) => &client.order,
        //     None => todo!()
        // };
        //
        // let native_options = eframe::NativeOptions::default();
        // eframe::run_native(
        //     &format!("backtest {:?}", self.strategy),
        //     native_options,
        //     Box::new(|cc| Box::new(visualization::candle::App{
        //         candle_data,
        //         cash_data,
        //         net_asset_data,
        //         order_data
        //     })),
        // ).expect("Plotting error");
    }
    pub fn set_data_feed(&mut self, symbol: &str) -> &mut Golden{
        self.data = match self.mode{
            GoldenModeType::Backtest => todo!(),
            GoldenModeType::Paper => todo!(),

            // GoldenModeType::Paper => self.broker.realtime_bars(&Contract {
            //     symbol: "USD".to_owned(),
            //     security_type: SecurityType::ForexPair,
            //     currency: "JPY".to_owned(),
            //     exchange: "IDEALPRO".to_owned(),
            //     ..Default::default()
            // }, BarSize::Sec5, WhatToShow::MidPoint, false).unwrap(),

            GoldenModeType::Live => todo!()
        };
        self
    }
    pub fn set_mode(&mut self, mode: GoldenModeType) -> &mut Golden{
        self.mode = mode;
        self
    }
    pub fn set_broker(&mut self, cash: f64) -> &mut Golden {
        self.broker = match self.mode {
            GoldenModeType::Backtest => BaseBroker{
                name: "backtest".to_owned(),
                client: None,
                backtest_client: Option::from(BackTestClient {
                    cash: Vec::from([cash]),
                    position: Vec::from([0.0]),
                    net_assets: Vec::from([cash]),
                    order: vec![],
                })

            },
            GoldenModeType::Paper => BaseBroker {
                name: "paper".to_owned(),
                client: Some(Client::connect("127.0.0.1:7497", 100).unwrap()),
                backtest_client: None
            },
            GoldenModeType::Live => todo!()
        };
        self
    }
    pub fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut Golden{
        self.strategy = strategy;
        self
    }
    // pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GoldenBuilder {
    //     self
    // }
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

#[derive(Debug)]
struct Row {
    label: String,
    values: Vec<i32>,
}

// https://docs.rs/csv/latest/csv/struct.Reader.html
pub fn example() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("data/SPY.csv")?;

    if let Some(result) = rdr.next() {
        let record: Row = result?;
        log::info!("{:?}", record);
        Ok(())
    } else {
        Err(From::from("expected at least one record but got none"))
    }
}

fn fetch_csv_data(symbol: &str) -> Result<String, Error> {
    let mut buffer = String::new();
    let mut csv_file = File::open(format!("data/{symbol}.csv")).unwrap();
    csv_file.read_to_string(&mut buffer).expect("TODO: panic message");
    if buffer.is_empty() {
        return Err(Error::CSVError);
    }
    Ok(buffer)
}

pub fn get_bar_from_csv(symbol: &str) -> Vec<Bar>{
    let buffer = fetch_csv_data(symbol).unwrap();
    let mut lines = buffer.lines();
    let _ = lines.next().unwrap(); // skip header

    let mut bars: Vec<Bar> = Vec::new();
    for line in lines {
        let r: Vec<_> = line.split(',').collect();
        bars.push(Bar{
                date: OffsetDateTime::now_utc(),
                open: r[2].parse().unwrap(),
                high: r[2].parse().unwrap(),
                low: r[3].parse().unwrap(),
                close: r[4].parse().unwrap(),
                volume: 0.0,
                wap: 0.0,
                count: 0,
        })
    }
    bars
    // csv_reader.map(move |r| Bar{
    //     date: OffsetDateTime::now_utc(),
    //     open: r.1[0],
    //     high: r.1[1],
    //     low: r.1[2],
    //     close: r.1[3],
    //     volume: 0.0,
    //     wap: 0.0,
    //     count: 0,
    // })
}

// fn connect_and_fetch_market_data() -> impl Iterator<Item = Bar>{
//