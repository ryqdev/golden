use std::fs;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use serde_derive::{Deserialize, Serialize};
use super::Command;
use std::process::exit;
use clickhouse::{ Client, Row};
use async_trait::async_trait;
use std::env;
use dotenv::dotenv;
use time::Date;


pub struct BackTestCommand;


#[async_trait]
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

    async fn handler(m: &ArgMatches) -> Result<()> {
        log::info!("handle backtest command");
        let project = m.get_one::<String>("project").unwrap();
        backtest(project).await?;
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


#[derive(Debug, Row, Serialize, Deserialize)]
struct Data{
    #[serde(with = "clickhouse::serde::time::date")]
    date: Date,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64
}


async fn backtest(project_path: &str) -> Result<()> {
    log::info!("Backtesting {}...", project_path);
    execute_strategy(parse_strategy(project_path).unwrap() ).await?;
    Ok(())
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

async fn parse_data(date: String) -> Result<Data> {
    log::info!("parsing data on {date}");
    dotenv().ok();
    let password = env::var("PASSWORD").expect("Cannot find password in .env file");
    let client = Client::default()
        .with_url("https://famep8kcv5.ap-southeast-1.aws.clickhouse.cloud:8443")
        .with_user("default")
        .with_password(password)
        .with_database("default");

    let data = fetch(&client, date).await?;
    log::info!("{:?}", data);

    Ok(data)
}


async fn fetch(client: &Client, date: String) -> Result<Data> {
    let mut cursor = client
        .query("select * from TLT3 where Date == ?")
        .bind(date)
        .fetch::<Data>()?;

    let Some(date) = cursor.next().await? else {panic!("db error")};

    Ok(date)
}


async fn execute_strategy(strategy: Strategy) -> Result<()> {
    log::info!("executing...");
    log::info!("strategy: {:?}", strategy);
    let start_price = parse_data(strategy.config.start).await?;
    let end_price = parse_data(strategy.config.end).await?;
    log::info!("P&L: {}:{}, {} -> {}", strategy.config.symbol, strategy.config.holding, start_price.close, end_price.close);
    Ok(())
}
