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
use crate::green::strategy::hold::SimpleStrategy;
use crate::green::visualization;


#[derive(Default)]
pub struct Green {
    data: Vec<Vec<f64>>,
    strategy: SimpleStrategy,
    // broker: BackTestBroker,
}


#[derive(Default)]
pub struct GreenBuilder {
    data: Vec<Vec<f64>>,
    strategy: SimpleStrategy,
    // broker: BackTestBroker
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
            // log::info!("\x1b[93m bar:\x1b[0m {:?} ", bar);
            self.strategy.next(bar);
            // let action = Action::Buy;
            //
            // // TODO: build the client
            // let order_id = client.next_order_id();
            // let order = order_builder::market_order(action, 1000.0);
            //
            // let notices = client.place_order(order_id, &contract, &order).unwrap();
            // for notice in notices {
            //     if let OrderNotification::ExecutionData(data) = notice {
            //         println!("{} {} shares of {}", data.execution.side, data.execution.shares, data.contract.symbol);
            //     } else {
            //         println!("{:?}", notice);
            //     }
            // }
        }
        log::info!("{}", self.strategy.cash.last().unwrap());
        log::info!("{}", self.strategy.position.last().unwrap());
        log::info!("{}", self.strategy.net_assets.last().unwrap());
    }
    pub fn plot(&self) {
        log::info!("Ploting {:?}...", self.strategy.name);
        let candle_data = self.data.clone();
        let cash_data = self.strategy.cash.clone();
        let net_asset_data = self.strategy.net_assets.clone();
        let order_data = self.strategy.order.clone();

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
    // fn run_strategy(&self){
    //     self.broker.set_cash(XXX)
    // }
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
    // pub fn add_broker(&mut self, cash: f64) -> &mut GreenBuilder {
    //     self.broker = BackTestBroker{
    //         cash: Vec::from([cash]),
    //         position: Vec::from([0.0]),
    //         net_assets:  Vec::from([cash]),
    //     };
    //     self
    // }
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
            // broker: self.broker.clone()
        })
    }
}