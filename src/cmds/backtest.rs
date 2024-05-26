use super::Command;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;
use time::Date;
use crate::green::{
    green::Green,
    feeds, strategy, broker
};
use crate::green::broker::backtest::BackTestBroker;
use crate::strategy::hold::SimpleStrategy;

pub struct BackTestCommand;

#[async_trait]
impl Command for BackTestCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("backtest")
            .about("back test strategies")
            .visible_alias("b")
            .arg(
                Arg::new("symbol")
                    .long("symbol")
                    .value_parser(clap::value_parser!(String))
                    .help("symbol")
                    .num_args(1),
            )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        let symbol = m.get_one::<String>("symbol").unwrap();
        log::info!("Handle backtest {symbol}");
        backtest(symbol).await?;
        Ok(())
    }

}

async fn backtest(symbol: &str) -> Result<()> {
    log::info!("Backtesting {symbol}...");
    let cash = 10_000.0;
    let mut green = Green::new()
        .add_data_feed(symbol)
        // .add_broker(100_000.0)
        .add_strategy(SimpleStrategy {
            name: "simple".to_string(),
            cash: Vec::from([cash]),
            position: Vec::from([0.0]),
            net_assets:  Vec::from([cash]),
            order: vec![],
        })
        .build();

    green.run();
    green.plot();
    Ok(())
}