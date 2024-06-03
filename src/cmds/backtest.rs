use super::{Command, BackTestGolden, Golden};
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use async_trait::async_trait;
pub struct BackTestCommand;
use crate::strategy::strategy::BaseStrategy;

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

        // TODO: use Builder?
        BackTestGolden::new()
            .set_broker(100_000.0)
            .set_data_feed(symbol)
            .set_strategy(BaseStrategy{})
            .run()
            .set_analyzer()
            .plot();
        Ok(())
    }

}

