use std::fmt::format;
use std::fs;
use anyhow::{Error, Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use clap::builder::Str;
use serde_derive::Deserialize;
use super::Command;
use std::process::exit;

pub struct BackTestCommand;

impl Command for BackTestCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("backtest")
            .about("Back test strategies")
            .visible_alias("b")
            .arg(
                Arg::new("project")
                    .long("project")
                    .value_parser(clap::value_parser!(String))
                    .help("project name")
                    .num_args(1),
            )
    }

    fn handler(m: &ArgMatches) -> Result<(), Error> {
        log::info!("handle backtest command");

        match m.get_one::<String>("project").map(String::as_str){
            Some(project_path) => backtest(project_path),
            None => (),
        }
        Ok(())
    }

}

#[derive(Deserialize, Debug)]
struct StrategyConfig{
    symbol: String,
    holding: usize,
    start: String,
    end: String
}

#[derive(Deserialize)]
struct Strategy {
    config: StrategyConfig,
}

fn backtest(project_path: &str) {
    log::info!("Backtesting {}...", project_path);
    parse_strategy(project_path);
}

fn parse_strategy(project: &str){
    let filename = format!("strategy/{}.toml", project);

    let contents = fs::read_to_string(filename.clone()).unwrap_or_else(|_| "WTF".to_string());

    let data: Strategy = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    println!("{:?}", data.config);
}