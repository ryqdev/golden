use clap::{ArgMatches, Command as ClapCommand};
use anyhow::Error;


pub trait Command {
    fn usage() -> ClapCommand;
    fn handler(m: &ArgMatches) -> Result<(), Error>;
}


mod data;

pub use data::DataCommand;