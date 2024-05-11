mod cmds;
use crate::{cmds::{Command, DataCommand, BackTestCommand}};

use std::io::Write;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Info)
        .init();

    let cmd = clap::Command::new("golden")
        .subcommands(vec![
            DataCommand::usage().display_order(1),
            BackTestCommand::usage().display_order(2),
        ]);


    match cmd.get_matches().subcommand() {
        Some(("data", sub_m)) => Ok(DataCommand::handler(sub_m).await?),
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m).await?),
        _ => Err(anyhow::Error::msg("match error!!!")),
    }

}
