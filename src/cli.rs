use crate::cmds::{
    Command,
    backtest::BackTestCommand,
    live::LiveTradingCommand,
    paper::PaperTradingCommand
};
use anyhow::Result;

pub async fn match_cmds() -> Result<()> {
    match clap::Command::new("golden")
        .subcommands(vec![
            BackTestCommand::usage().display_order(1),
            PaperTradingCommand::usage().display_order(2),
            LiveTradingCommand::usage().display_order(3)
        ])
        .arg_required_else_help(true)
        .get_matches()
        .subcommand()
    {
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m).await?),
        Some(("paper-trading", sub_m)) => Ok(PaperTradingCommand::handler(sub_m).await?),
        Some(("live-trading", sub_m)) => Ok(LiveTradingCommand::handler(sub_m).await?),
        _ => Err(anyhow::Error::msg("Match commands fails")),
    }
}