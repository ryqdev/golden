use std::{
    fs,
    process::exit
};
use super::Command;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use clickhouse::{ Client, Row};
use async_trait::async_trait;
use time::Date;
use crate::green::{
    Green, feeds, strategy,
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
    let green = Green::new()?;

    // green.add_strategy(BuyAndHold);

    // let date_feed: Box<dyn BaseData> = feeds::yahoo::YahooFinanceData{
    //     csv_file_path: "".to_string(),
    //     start_date: "".to_string(),
    //     end_date: "".to_string()
    // };
    // green.add_data_feed();
    green.run();
    green.plot();
    Ok(())
}