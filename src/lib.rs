//! # Quick Install
//! ```shell
//! cargo install golden
//! ```
//!
//! ## Useful Commands
//! ```shell
//! # download csv to data/
//! golden csv --symbol SPY
//!
//! # backtest
//! golden backtest --symbol SPY
//!
//! # paper trading
//! golden paper --broker ibkr
//!
//! ```


#![deny(missing_docs)]

pub mod cmds;
pub mod feeds;
pub mod broker;
pub mod cli;

mod err;
mod analyzer;
mod visualization;
mod monitor;
mod color;
