use clap::{Arg, ArgMatches, Command as ClapCommand};
use super::Command;
use anyhow::Result;
use async_trait::async_trait;

pub struct CSVCommand;

#[async_trait]
impl Command for CSVCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("csv")
            .about("download csv file")
            .visible_alias("c")
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
        log::info!("Download {symbol} data");

        Ok(())

    }
}
