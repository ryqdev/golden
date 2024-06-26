use super::{Command, BackTestGolden, Golden, parse_config, strategy_mapping};
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use async_trait::async_trait;
use crate::feeds::csv::fetch::get_bar_from_yahoo;

pub struct BackTestCommand;
use crate::strategy::strategy::BaseStrategy;

#[async_trait]
impl Command for BackTestCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("backtest")
            .about("back test strategies")
            .visible_alias("b")
            .arg(
                Arg::new("config")
                    .long("config")
                    .value_parser(clap::value_parser!(String))
                    .help("config file path")
                    .num_args(1),
            )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        let config_file_path = match m.get_one::<String>("config") {
            Some(c) => {c}
            None => {"config.toml"} // config.toml by default
        };
        let toml_data = parse_config(config_file_path)?;
        log::info!("Backtest {:?}", toml_data);

        get_bar_from_yahoo(&toml_data.config.symbol, true).await?;

        BackTestGolden::new()
            .set_broker(toml_data.config.cash)
            .set_data_feed(&toml_data.config.symbol)
            .set_strategy(strategy_mapping(&toml_data.config.strategy))
            .run()
            .set_analyzer()
            .plot(toml_data.config.symbol);
        Ok(())
    }

}

