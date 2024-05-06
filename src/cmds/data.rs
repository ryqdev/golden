use super::Command;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use anyhow::Error;

pub struct DataCommand;

impl Command for DataCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("data")
            .about("Manage equity data")
            .visible_alias("d")
            .arg(
                Arg::new("download")
                    .display_order(1)
                    .short('d')
                    .long("download")
                    .help("Download data")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("update")
                    .display_order(2)
                    .short('u')
                    .long("update")
                    .help("Update data")
                    .action(ArgAction::SetTrue),
            )
    }

    fn handler(m: &ArgMatches) -> Result<(), Error> {
        log::info!("handling...");
        Ok(())
    }

}
