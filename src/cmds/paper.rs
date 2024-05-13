use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};
use crate::cmds::Command;

pub struct PaperTradingCommand;

#[async_trait]
impl Command for PaperTradingCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("paper-trading")
            .about("Paper trading")
            .visible_alias("p")
    }

    async fn handler(m: &ArgMatches) -> anyhow::Result<()> {
        log::info!("handle paper trading");
        Ok(())
    }
}