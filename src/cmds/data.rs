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
                    .action(ArgAction::Set),
            )
            .arg(
                Arg::new("update")
                    .display_order(2)
                    .short('u')
                    .long("update")
                    .help("Update data")
                    .action(ArgAction::Set),
            )
    }

    fn handler(m: &ArgMatches) -> Result<(), Error> {
        log::info!("handle data command");
        if m.contains_id("download") {
            match m.get_one::<String>("download").map(String::as_str){
                Some(equity) => log::info!("Downloading {:?}...", equity),
                None => (),
            }
        }
        if m.contains_id("update") {
            match m.get_one::<String>("update").map(String::as_str){
                Some(equity) => log::info!("Updating {:?}...", equity),
                None => (),
            }
        }
        Ok(())
    }
}
