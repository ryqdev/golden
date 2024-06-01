use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};
use crate::cmds::Command;

pub struct PaperTradingCommand;

use std::collections::VecDeque;

use ibapi::contracts::{Contract, SecurityType};
use ibapi::market_data::realtime::{BarSize, Bar, WhatToShow};
use ibapi::orders::{order_builder, Action, OrderNotification};
use ibapi::Client;

struct BreakoutChannel {
    ticks: VecDeque<(f64, f64)>,
    size: usize,
}

impl BreakoutChannel {
    fn new(size: usize) -> BreakoutChannel {
        BreakoutChannel {
            ticks: VecDeque::with_capacity(size + 1),
            size,
        }
    }

    fn ready(&self) -> bool {
        self.ticks.len() >= self.size
    }

    fn add_bar(&mut self, bar: &Bar) {
        self.ticks.push_back((bar.high, bar.low));

        if self.ticks.len() > self.size {
            self.ticks.pop_front();
        }
    }

    fn high(&self) -> f64 {
        self.ticks.iter().map(|x| x.0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn low(&self) -> f64 {
        self.ticks.iter().map(|x| x.1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }
}

async fn paper_trading(){
    log::info!("Paper trading...");

    let client = Client::connect("127.0.0.1:7497", 100).unwrap();

    // let contract = Contract {
    //     symbol: "TSLA".to_string(),
    //     security_type: SecurityType::Stock,
    //     currency: "USD".to_string(),
    //     exchange: "SMART".to_string(),
    //     ..Default::default()
    // };


    let contract = Contract {
        symbol: "USD".to_owned(),
        security_type: SecurityType::ForexPair,
        currency: "JPY".to_owned(),
        exchange: "IDEALPRO".to_owned(),
        ..Default::default()
    };

    log::info!("{:?}", contract);

    let bars = client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::MidPoint, false).unwrap();
    let mut channel = BreakoutChannel::new(30);
    for bar in bars {
        log::info!("\x1b[93m bar:\x1b[0m {:?} ", bar);
        channel.add_bar(&bar);

        // Ensure enough bars and no open positions.
        if !channel.ready() {
            continue;
        }

        let action = if bar.close > channel.high() {
            Action::Sell
        } else if bar.close < channel.low() {
            Action::Buy
        } else {
            continue;
        };

        let order_id = client.next_order_id();
        let order = order_builder::market_order(action, 1000.0);

        let notices = client.place_order(order_id, &contract, &order).unwrap();
        for notice in notices {
            if let OrderNotification::ExecutionData(data) = notice {
                println!("{} {} shares of {}", data.execution.side, data.execution.shares, data.contract.symbol);
            } else {
                println!("{:?}", notice);
            }
        }
    }

}

#[async_trait]
impl Command for PaperTradingCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("paper")
            .about("Paper trading")
            .visible_alias("p")
    }

    async fn handler(m: &ArgMatches) -> anyhow::Result<()> {
        log::info!("handle paper trading");
        paper_trading().await;
        Ok(())
    }
}