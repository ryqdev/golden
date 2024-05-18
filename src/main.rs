use std::io::Write;

mod cmds;
use crate::{cmds::{Command,
                   backtest::BackTestCommand,
                   paper::PaperTradingCommand}
};

fn init_log() {
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    init_log();

    let cmd = clap::Command::new("golden")
        .subcommands(vec![
            BackTestCommand::usage().display_order(1),
            PaperTradingCommand::usage().display_order(2)
        ]);

    match cmd.get_matches().subcommand() {
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m).await?),
        Some(("paper-trading", sub_m)) => Ok(PaperTradingCommand::handler(sub_m).await?),
        _ => Err(anyhow::Error::msg("match error!!!")),
    }
}
