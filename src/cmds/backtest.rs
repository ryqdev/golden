use anyhow::Error;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use super::Command;

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
            Some(project_path) => log::info!("Backtesting {:?}...", project_path),
            None => (),
        }
        Ok(())
    }
}