use std::io::Read;
use clap::{Arg, ArgMatches, Command as ClapCommand};
use super::Command;
use anyhow::Result;
use async_trait::async_trait;

pub struct CSVCommand;

#[async_trait]
impl Command for CSVCommand {
    fn usage() -> ClapCommand {
        ClapCommand::new("csv")
            .about("download csv file")
            .visible_alias("c")
            .arg(
                Arg::new("symbol")
                    .long("symbol")
                    .value_parser(clap::value_parser!(String))
                    .help("symbol")
                    .num_args(1),
            )
    }

    /// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
    /// Example url to download historial csv data: https://query1.finance.yahoo.com/v7/finance/download/TLT?period1=345479400&period2=1717257709&interval=1d&events=history&includeAdjustedClose=true
    /// It works well in brower `but` met Status 429 with `reqwest` GET RESTful request.
    /// After learn the source code of https://github.com/ranaroussi/yfinance.
    /// I notice that I should use https://query1.finance.yahoo.com/v8/finance/chart/AAPL fetch the chart data rather than access the download url
    ///
    ///
    /// https://stackoverflow.com/questions/78111453/yahoo-finance-api-file-get-contents-429-too-many-requests
    /// add User Agent to solve `429` error
    async fn handler(m: &ArgMatches) -> Result<()> {
        let symbol = m.get_one::<String>("symbol").unwrap();
        log::info!("Download {symbol} data");
        let url = "https://query1.finance.yahoo.com/v7/finance/download/TLT?period1=345479400&period2=1717257709&interval=1d&events=history&includeAdjustedClose=true";

        let client = reqwest::Client::builder()
            .user_agent("curl/7.68.0")
            .build()?;

        let response = client.get(url).send().await?;
        // log::info!("response: {:?}", response);
        //
        // log::info!("Status Code: {}", response.status());
        let response_body = response.text().await?;
        log::info!("Response body: {}", response_body);
        Ok(())
    }
}
