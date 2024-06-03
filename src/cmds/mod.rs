use clap::{ArgMatches, Command as ClapCommand};
use anyhow::Result;
use async_trait::async_trait;
use crate::broker::backtest::backtest::BacktestBroker;
use crate::feeds::{
    Bar,
    csv::fetch::{
        get_bar_from_csv,
        get_bar_from_yahoo
    }
};
use crate::visualization;
use crate::strategy::strategy::BaseStrategy;
use crate::broker::backtest::backtest::Action;
use crate::color::GoldenColor;

#[async_trait]
pub trait Command {
    fn usage() -> ClapCommand;
    async fn handler(m: &ArgMatches) -> Result<()>;
}

trait Golden{
    // https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait
    fn new() -> Box<dyn Golden> where Self: Sized;
    fn run(&mut self) -> &mut dyn Golden;// the last step
    fn set_data_feed(&mut self, symbol: &str) -> &mut dyn Golden;
    fn set_broker(&mut self, cash: f64) -> &mut dyn Golden;
    fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut dyn Golden;
    fn set_analyzer(&mut self) -> &mut dyn Golden;
    fn set_monitor(&mut self) -> &mut dyn Golden;
    fn plot(&self);
}

pub struct BackTestGolden {
    data: Vec<Bar>,
    strategy: BaseStrategy,
    broker: BacktestBroker,
}


impl Golden for BackTestGolden {
    fn new() -> Box<dyn Golden>
        where Self: Sized
    {
        log::info!("Get BackTestGolden");
        Box::new(BackTestGolden {
            data: Vec::new(),
            strategy: BaseStrategy::default(),
            broker: BacktestBroker::default(),
        })
    }

    fn run(&mut self) -> &mut dyn Golden{
        log::info!("Running {:?}...", self.strategy);

        for bar in self.data.iter() {
            let order = self.strategy.next(&bar);
            let cash = self.broker.cash.last().unwrap();
            let position = self.broker.position.last().unwrap();

            match order.action {
                Action::Buy => {
                    log::info!("Buy: {:?}", order);
                    self.broker.cash.push(cash - order.size * bar.close);
                    self.broker.position.push(position + order.size);
                    self.broker.order.push(order);
                }
                Action::Sell => {
                    log::info!("Sell: {:?}", order);
                    self.broker.cash.push(cash + order.size * bar.close);
                    self.broker.position.push(position - order.size);
                    self.broker.order.push(order);
                }
                _ => {
                    self.broker.cash.push(*cash);
                    self.broker.position.push(*position);
                }
            }
            self.broker.net_assets.push(self.broker.cash.last().unwrap() + self.broker.position.last().unwrap() * bar.close)
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

    fn set_analyzer(&mut self) -> &mut dyn Golden {
        // very simple analyzer
        let p_h = self.broker.net_assets.last().unwrap() - self.broker.net_assets.first().unwrap();
        let color = if p_h > 0.0 {GoldenColor::GREEN} else {GoldenColor::RED};
        let reset_color = GoldenColor::RESET;
        log::info!("{color}P&H: {p_h} {reset_color}");
        self
    }

    fn set_monitor(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn plot(&self) {
        log::info!("Plotting {:?}...", self.strategy);
        let candle_data = self.data.clone();
        let cash_data = self.broker.cash.clone();
        let order_data = self.broker.order.clone();
        let net_assets_data = self.broker.net_assets.clone();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("backtest {:?}", self.strategy),
            native_options,
            Box::new(|_| Box::new(visualization::vis::App{
                candle_data,
                cash_data,
                net_assets_data,
                order_data
            })),
        ).expect("Plotting error");
    }
}

struct PaperGolden {}

impl Golden for PaperGolden{
    fn new() -> Box<dyn Golden> where Self: Sized {
        todo!()
    }

    fn run(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn set_data_feed(&mut self, symbol: &str) -> &mut dyn Golden {
        todo!()
    }

    fn set_broker(&mut self, cash: f64) -> &mut dyn Golden {
        todo!()
    }

    fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut dyn Golden {
        todo!()
    }

    fn set_analyzer(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn set_monitor(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn plot(&self) {
        todo!()
    }
}

struct LiveGolden{}

impl Golden for LiveGolden {
    fn new() -> Box<dyn Golden> where Self: Sized {
        todo!()
    }

    fn run(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn set_data_feed(&mut self, symbol: &str) -> &mut dyn Golden {
        todo!()
    }

    fn set_broker(&mut self, cash: f64) -> &mut dyn Golden {
        todo!()
    }

    fn set_strategy(&mut self, strategy: BaseStrategy) -> &mut dyn Golden {
        todo!()
    }

    fn set_analyzer(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn set_monitor(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn plot(&self) {
        todo!()
    }
}


pub mod backtest;
pub mod paper;
pub mod live;
pub mod csv;
