use crate::cmds::backtest::BackTestCommand;
use crate::cmds::Command;
use crate::cmds::live::LiveTradingCommand;
use crate::cmds::paper::PaperTradingCommand;


pub async fn match_cmds() -> anyhow::Result<()> {
    match clap::Command::new("golden")
        .subcommands(vec![
            BackTestCommand::usage().display_order(1),
            PaperTradingCommand::usage().display_order(2),
            LiveTradingCommand::usage().display_order(3)
        ])
        .get_matches()
        .subcommand()
    {
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m).await?),
        Some(("paper-trading", sub_m)) => Ok(PaperTradingCommand::handler(sub_m).await?),
        Some(("live-trading", sub_m)) => Ok(LiveTradingCommand::handler(sub_m).await?),
        _ => Err(anyhow::Error::msg("Miss arguments. Please open Makefile to get instructions")),
    }
}