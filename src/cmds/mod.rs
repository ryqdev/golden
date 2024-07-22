use clap::{ArgMatches, Command as ClapCommand};
use anyhow::Result;
use async_trait::async_trait;
use crate::broker::backtest::backtest::BacktestBroker;
use crate::feeds::{
    Bar,
    csv::fetch::{
        get_bar_from_csv,
        get_bar_from_yahoo,
        get_close_price_from_csv
    }
};
use crate::visualization;
use crate::strategy::BaseStrategy;
use crate::broker::backtest::backtest::Action;
use crate::color::GoldenColor;
use serde_derive::Deserialize;
use std::fs;
use eframe::egui_glow::check_for_gl_error_even_in_release;


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
    fn plot(&self, symbol: String);
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
            let buying_power = (cash / bar.close / 100.0) as i64 ;
            log::info!("{}: Cash: {:?}, Position: {:?}, Close Price: {}", bar.date, cash, position, bar.close);
            match order.action {
                Action::Buy => {
                    log::info!("Buy: {:?}", order);
                    if buying_power < 1 {
                        log::error!("buying power is smaller than 1. Cannot buy");
                        self.broker.cash.push(*cash);
                        self.broker.position.push(*position);
                        self.broker.order.push(order);
                    } else {
                        self.broker.cash.push(cash - (buying_power * 100 ) as f64 * bar.close);
                        self.broker.position.push(position + (buying_power * 100 ) as f64);
                        self.broker.order.push(order);
                    }
                }
                Action::Sell => {
                    log::info!("Sell: {:?}", order);
                    if *position <= 0.0 {
                        log::error!("position is smaller than 0. Cannot sell");
                        self.broker.cash.push(*cash);
                        self.broker.position.push(*position);
                        self.broker.order.push(order);
                    } else {
                        self.broker.cash.push(cash + position * bar.close);
                        self.broker.position.push(0.0);
                        self.broker.order.push(order);
                    }
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
        log::info!("{color}P&H Percent: {}% {reset_color}", 100.0 * p_h / self.broker.cash[0]);

        let base_line_Percent = 100.0 * (self.data.last().unwrap().close - self.data.first().unwrap().close) / self.data.first().unwrap().close;
        log::info!("beta: {base_line_Percent}%");

        let alpha =  100.0 * p_h / self.broker.cash[0] - base_line_Percent;
        let color = if alpha > 0.0 {GoldenColor::GREEN} else {GoldenColor::RED};
        log::info!("{color}Alpha: {alpha}% {reset_color}");
        self
    }

    fn set_monitor(&mut self) -> &mut dyn Golden {
        todo!()
    }

    fn plot(&self, symbol: String) {
        log::info!("Plotting {:?}...", self.strategy);
        let candle_data = self.data.clone();
        let cash_data = self.broker.cash.clone();
        let base_line_data = get_close_price_from_csv(&symbol).unwrap();
        let order_data = self.broker.order.clone();
        let net_assets_data = self.broker.net_assets.clone();

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            &format!("backtest {:?}", self.strategy),
            native_options,
            Box::new(|_| Box::new(visualization::vis::App{
                symbol,
                candle_data,
                cash_data,
                base_line_data,
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

    fn plot(&self, symbol: String) {
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

    fn plot(&self, symbol: String) {
        todo!()
    }
}

// Top level struct to hold the TOML data.
#[derive(Deserialize, Debug)]
struct TomlData {
    config: Config,
}

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug)]
struct Config {
    broker: String,
    symbol: String,
    time: String,
    strategy: String,
    cash: f64
}


pub fn parse_config(config_file_path: &str) -> Result<TomlData>{
    log::info!("start parsing config from {config_file_path}");
    let config_data: TomlData = toml::from_str(&*fs::read_to_string(config_file_path)?)?;
    log::info!("{:?}", config_data);
    Ok(config_data)
}

pub fn strategy_mapping(_strategy: &str) -> BaseStrategy {
    // TODO
    return BaseStrategy{}
}

pub mod backtest;
pub mod paper;
pub mod live;
pub mod csv;
