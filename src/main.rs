mod cmds;

use log::LevelFilter;
use crate::{
    cmds::{
        Command,
        DataCommand
    }
};
use anyhow::Error;


fn main() -> Result<(),Error> {
    let cmd = clap::Command::new("golden")
        .subcommands(vec![
            DataCommand::usage().display_order(1),
        ]);
    let m = cmd.clone().get_matches();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .init();

    match m.subcommand() {
        Some(("data", sub_m)) => Ok(DataCommand::handler(sub_m)?),
        _ => Err(Error::msg("match error!!!")),
    }

}
