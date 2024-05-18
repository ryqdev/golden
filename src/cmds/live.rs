use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};
use crate::cmds::Command;

pub struct LiveTradingCommand;

#[async_trait]
impl Command for LiveTradingCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("live-trading")
            .about("Live trading")
            .visible_alias("l")
    }

    async fn handler(m: &ArgMatches) -> anyhow::Result<()> {
        log::info!("handle live trading");
        Ok(())
    }
}