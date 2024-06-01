use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use crate::cmds::Command;

pub struct PaperTradingCommand;

use crate::broker::{
    ibkr::trade::ibkr_trading,
    alpaca::alpaca::alpaca_trading
};
use anyhow::Result;

enum BrokerType {
    IBKR,
    ALPACA
}

impl BrokerType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "IBKR" => Option::from(BrokerType::IBKR),
            "ALPACA" => Option::from(BrokerType::ALPACA),
            _ => None
        }
    }
}

#[async_trait]
impl Command for PaperTradingCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("paper")
            .about("Paper trading")
            .visible_alias("p")
            .arg(
                Arg::new("broker")
                    .long("broker")
                    .value_parser(clap::value_parser!(String))
                    .help("broker")
                    .num_args(1),
            )

    }

    async fn handler(m: &ArgMatches) -> Result<()> {
        let broker = m.get_one::<String>("broker").unwrap();
        log::info!("Paper trading on {broker}");
        match BrokerType::from_str(broker) {
            Some(BrokerType::IBKR) => ibkr_trading().await,
            Some(BrokerType::ALPACA) => alpaca_trading().await,
            _ => {
                log::error!("Broker not matched!")
            }
        };
        Ok(())
    }
}