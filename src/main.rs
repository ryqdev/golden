mod cmds;

use log::LevelFilter;
use crate::{
    cmds::{
        Command,
        DataCommand,
        BackTestCommand
    }
};
use anyhow::Error;


fn main() -> Result<(),Error> {
    let cmd = clap::Command::new("golden")
        .subcommands(vec![
            DataCommand::usage().display_order(1),
            BackTestCommand::usage().display_order(2),
        ]);

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .init();

    match cmd.get_matches().subcommand() {
        Some(("data", sub_m)) => Ok(DataCommand::handler(sub_m)?),
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m)?),
        _ => Err(Error::msg("match error!!!")),
    }

}
