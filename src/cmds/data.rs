use super::Command;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use anyhow::Result;
use async_trait::async_trait;


pub struct DataCommand;

#[async_trait]
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
                    .help("download data")
                    .action(ArgAction::Set),
            )
    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        log::info!("handle data command");
        if m.contains_id("download") {
            match m.get_one::<String>("download").map(String::as_str){
                Some(equity) => log::info!("download {:?}...", equity),
                None => (),
            }
        }
        Ok(())
    }
}
