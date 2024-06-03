use std::collections::VecDeque;

use ibapi::contracts::{Contract, SecurityType};
use ibapi::market_data::realtime::{BarSize, Bar, WhatToShow};
use ibapi::orders::{order_builder, Action, OrderNotification};
use ibapi::Client;
use time::OffsetDateTime;


/// https://github.com/wboayue/rust-ibapi
/// But his work is not finished, lacking some features, e.g.:
/// 1. Contract::stock() only supports stock, futures and crypto.
/// 2. BarSize only supports Sec5 type
pub async fn ibkr_trading(){
    // TODO: use PaperGolden
    log::info!("trade with ibkr");

    // connect my local IBKR TWS
    let client = Client::connect("127.0.0.1:7497", 100).unwrap();

    let contract = Contract {
        symbol: "USD".to_owned(),
        security_type: SecurityType::ForexPair,
        currency: "JPY".to_owned(),
        exchange: "IDEALPRO".to_owned(),
        ..Default::default()
    };

    log::info!("{:?}", contract);
    let mut previous_bar = Bar{
        date: OffsetDateTime::now_utc(),
        open: 0.0,
        high: 0.0,
        low: 0.0,
        close: 0.0,
        volume: 0.0,
        wap: 0.0,
        count: 0,
    };

    let bars = client.realtime_bars(&contract, BarSize::Sec5, WhatToShow::MidPoint, false).unwrap();
    for bar in bars {
        log::info!("\x1b[93m bar:\x1b[0m {:?} ", bar);

        if previous_bar.close == 0.0 {
            previous_bar = bar;
            continue;
        }

        // a very simple implementation of mean-revesion
        // -ln(today's open / yesterday's close)

        let action = if bar.open > previous_bar.close {
            Action::Sell
        } else if bar.open < previous_bar.close{
            Action::Buy
        } else {
            continue;
        };

        previous_bar = bar;

        let order_id = client.next_order_id();
        let order = order_builder::market_order(action, 10000.0);

        let notices = client.place_order(order_id, &contract, &order).unwrap();
        for notice in notices {
            if let OrderNotification::ExecutionData(data) = notice {
                log::info!("{} {} shares of {}", data.execution.side, data.execution.shares, data.contract.symbol);
            } else {
                log::info!("{:?}", notice);
            }
        }
    }

}
