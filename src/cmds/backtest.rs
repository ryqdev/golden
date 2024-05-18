use std::fs;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use super::Command;
use std::process::exit;
use clickhouse::{ Client, Row};
use async_trait::async_trait;
use time::Date;
use crate::green::{Green, feeds, strategy};
use crate::green::feeds::BaseData;
use crate::strategy::hold::BuyAndHold;


pub struct BackTestCommand;


#[async_trait]
impl Command for BackTestCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("backtest")
            .about("back test strategies")
            .visible_alias("b")
            .arg(
                Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(String))
                    .help("project name")
                    .num_args(1),
            )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        log::info!("handle backtest command");
        let project = m.get_one::<String>("project").unwrap();
        backtest(project).await?;
        Ok(())
    }

}


async fn backtest(project_path: &str) -> Result<()> {
    log::info!("Backtesting {}...", project_path);
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