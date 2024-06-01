use crate::cmds::{
    Command,
    backtest::BackTestCommand,
    live::LiveTradingCommand,
    paper::PaperTradingCommand,
    csv::CSVCommand
};
use anyhow::Result;

pub async fn match_cmds() -> Result<()> {
    match clap::Command::new("golden")
        .subcommands(vec![
            BackTestCommand::usage().display_order(1),
            PaperTradingCommand::usage().display_order(2),
            LiveTradingCommand::usage().display_order(3),
            CSVCommand::usage().display_order(4)
        ])
        .arg_required_else_help(true)
        .get_matches()
        .subcommand()
    {
        Some(("backtest", sub_m)) => Ok(BackTestCommand::handler(sub_m).await?),
        Some(("paper", sub_m)) => Ok(PaperTradingCommand::handler(sub_m).await?),
        Some(("live", sub_m)) => Ok(LiveTradingCommand::handler(sub_m).await?),
        Some(("csv", sub_m)) => Ok(CSVCommand::handler(sub_m).await?),
        _ => Err(anyhow::Error::msg("Match commands fails")),
    }
}