use clap::{ArgMatches, Command as ClapCommand};
use anyhow::Result;
use async_trait::async_trait;


#[async_trait]
pub trait Command {
    fn usage() -> ClapCommand;
    async fn handler(m: &ArgMatches) -> Result<()>;
}


pub mod backtest;
pub mod paper;
pub mod live;
