use super::Command;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use async_trait::async_trait;
use time::Date;
use crate::green::{
    Green, feeds, strategy, broker
};
use crate::strategy::hold::BuyAndHold;

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
        log::info!("handle backtest");
        let symbol = m.get_one::<String>("symbol").unwrap();
        backtest(symbol).await?;
        Ok(())
    }

}


async fn backtest(symbol: &str) -> Result<()> {
    log::info!("Backtesting {}...", symbol);
    let green = Green::new()
        .add_data_feed("TLT")
        .add_strategy(BuyAndHold{})
        .build();

    green.run();
    green.plot();
    Ok(())
}