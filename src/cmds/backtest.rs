use super::Command;
use anyhow::{Result};
use clap::{Arg, ArgMatches, Command as ClapCommand};
use async_trait::async_trait;
use crate::{BaseStrategy, Golden, GoldenModeType};

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
    // TODO: Lifetime!!!!
    let green = Golden::new(GoldenModeType::Backtest, 100_000.0, symbol, BaseStrategy{ name: "test".to_string() });
        // .set_mode(GoldenModeType::Backtest)
        // .set_broker(100_000.0)
        // .set_data_feed(symbol)
        // .set_strategy(BaseStrategy{ name: "test".to_string() });

    green.run();
    green.plot();
    Ok(())
}