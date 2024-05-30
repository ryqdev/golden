use super::Command;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;
use time::Date;
use crate::green::{
    green::Green,
    feeds, broker
};
use crate::green::broker::backtest::BackTestBroker;
use crate::green::green::DataType::Backtest;
use crate::green::green::GreenModeType;
use crate::green::strategy::hold::SimpleStrategy;

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
        log::info!("Backtest {symbol}");
        backtest(symbol).await?;
        Ok(())
    }

}

async fn backtest(symbol: &str) -> Result<()> {
    let mut green = Green::new()
        .set_mode(GreenModeType::Backtest)
        .set_broker(100_000.0)
        .set_data_feed(symbol)
        .set_strategy(SimpleStrategy{})
        .build();

    green.run();
    green.plot();
    Ok(())
}