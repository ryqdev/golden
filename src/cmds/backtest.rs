use std::fmt::format;
use std::fs;
use anyhow::{Error, Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use clap::builder::Str;
use serde_derive::Deserialize;
use super::Command;
use std::process::exit;
use std::fs::File;
use log::info;


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

#[derive(Deserialize, Debug)]
struct Strategy {
    config: StrategyConfig,
}

#[derive(Deserialize, Debug)]
struct Data{
    date: String,
    open: String,
    high: String,
    low: String,
    close: String,
    adj_close: String,
    volume: String
}

fn backtest(project_path: &str) {
    log::info!("Backtesting {}...", project_path);
    let strategy = match parse_strategy(project_path) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("???");
            exit(1);
        }
    };

    log::info!("{:?}", strategy);

    let data = parse_data().unwrap();
    execute_strategy(strategy, data)
}

fn parse_strategy(project: &str) -> Result<Strategy>{
    let filename = format!("strategy/{}.toml", project);

    let contents = fs::read_to_string(filename.clone()).unwrap_or_else(|_| "WTF".to_string());

    let data: Strategy = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    Ok(data)
}

fn parse_data() -> Result<Vec<Data>> {
    log::info!("parsing data");
    let filename = "data/TLT.csv";
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut data_list :Vec<Data> = Vec::new();

    for result in rdr.records().into_iter() {
        let mut data_frame = Data{
            date: "".to_string(),
            open: "".to_string(),
            high: "".to_string(),
            low: "".to_string(),
            close: "".to_string(),
            adj_close: "".to_string(),
            volume: "".to_string(),
        };
        let record = result?;
        data_frame.date = record[0].to_string();
        data_frame.open= record[1].to_string();
        data_frame.high = record[2].to_string();
        data_frame.low = record[3].to_string();
        data_frame.close = record[4].to_string();
        data_frame.adj_close= record[5].to_string();
        data_frame.volume = record[6].to_string();
        data_list.push(data_frame);
    }
    // log::info!("{:?}", data_list);
    Ok(data_list)
}


fn execute_strategy(strategy: Strategy, data: Vec<Data>)  {
    log::info!("executing...");
    let start_price = &data[0].close;
    let end_price = &data[data.len() - 1].close;
    log::info!("{} - {}", start_price, end_price)
}
