// TODO: doc

pub mod cmds;
mod err;
pub mod feeds;
pub mod broker;
mod analyzer;
mod visualization;
pub mod strategy;

use std::fs::File;

use time::OffsetDateTime;

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


#[derive(Default)]
struct BacktestBroker {
    pub cash: Vec<f64>,
    pub position: Vec<f64>,
    pub net_assets: Vec<f64>,
    pub order: Vec<Order>
}

#[derive(Debug,Default)]
pub struct BaseStrategy{
    name: String,
}

impl BaseStrategy {
    fn next(&self, data: &Bar) -> Order {
        if data.close > data.open {
            Order{
                action: Action::Buy,
                size: 1.0
            }
        } else {
            Order{
                action: Action::Sell,
                size: 1.0
            }
        }
    }

}

trait Golden{
    // TODO: what is Sized
    fn new() -> Box<dyn Golden> where Self: Sized;
    fn run(&mut self) -> &mut dyn Golden;// the last step
    fn set_data_feed(&mut self, symbol: &str) -> &mut dyn Golden;
    fn set_broker(&mut self, cash: f64) -> &mut dyn Golden;
    fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut dyn Golden;
    fn plot(&self);
}


struct BackTestGolden {
    data: Vec<Bar>,
    strategy: BaseStrategy,
    broker: BacktestBroker,
}

impl Golden for BackTestGolden {
    // TODO: referece or not
    fn new() -> Box<dyn Golden>
    // TODO: what is Sized
    where Self: Sized
    {
        log::info!("Get BackTestGolden");
        Box::new(BackTestGolden {
            data: Vec::new(),
            strategy: BaseStrategy::default(),
            broker: BacktestBroker::default(),
        })
    }

    // TODO: why not pub?
    fn run(&mut self) -> &mut dyn Golden{
        log::info!("Running {:?}...", self.strategy);

        for bar in self.data.iter() {
            let order = self.strategy.next(&bar);
            let cash = self.broker.cash.last().unwrap();
            let position = self.broker.position.last().unwrap();

            match order.action {
                Action::Buy => {
                    log::info!("Buy: {:?}", order);
                    // TODO: when to use reference?
                    self.broker.cash.push(cash - &order.size * bar.close);
                    self.broker.position.push(position + &order.size);
                    self.broker.order.push(order);
                }
                Action::Sell => {
                    log::info!("Sell: {:?}", order);
                    self.broker.cash.push(cash + &order.size * bar.close);
                    self.broker.position.push(position - &order.size);
                    self.broker.order.push(order);
                }
                _ => {
                    // TODO: * or to_owned()
                    self.broker.cash.push(*cash);
                    self.broker.position.push(*position);
                }
            }
        }
        self
    }

    fn set_data_feed(&mut self, symbol: &str) -> &mut dyn Golden{
        log::info!("set data feed for {}", symbol);
        self.data = get_bar_from_csv(symbol).unwrap();
        self
    }
    fn set_broker(&mut self, cash: f64) -> &mut dyn Golden {
        log::info!("set broker");
        self.broker = BacktestBroker {
            cash: Vec::from([cash]),
            position: Vec::from([0.0]),
            net_assets: Vec::from([cash]),
            order: vec![],
        };
        self
    }
    fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut dyn Golden{
        log::info!("set strategy");
        self.strategy = strategy;
        self
    }
    // the last step
    fn plot(&self) {
        log::info!("Plotting {:?}...", self.strategy);
        let candle_data = self.data.clone();
        let cash_data = self.broker.cash.clone();
        let order_data = self.broker.order.clone();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("backtest {:?}", self.strategy),
            native_options,
            Box::new(|cc| Box::new(visualization::candle::App{
                candle_data,
                cash_data,
                order_data
            })),
        ).expect("Plotting error");
    }
    // pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer>) -> &GoldenBuilder {
    //     self
    // }

}

// struct PaperGolden {}
//
// impl Golden for PaperGolden{}
//
// struct LiveGolden{}
//
// impl Golden for LiveGolden {}


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

#[derive(Debug, serde::Deserialize, PartialEq)]
struct CSVData {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64
}

// https://docs.rs/csv/latest/csv/struct.Reader.html
pub fn get_bar_from_csv(symbol: &str) -> Result<Vec<Bar>, Box<dyn std::error::Error>> {
    csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader( File::open(format!("data/{symbol}.csv"))?)
        .deserialize::<CSVData>().map(|line| {
        let record = line?;
        Ok(Bar {
            date: OffsetDateTime::now_utc(),
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            volume: 0.0,
            wap: 0.0,
            count: 0,
        })
    }).collect()
}

// fn connect_and_fetch_market_data() -> impl Iterator<Item = Bar>{
//