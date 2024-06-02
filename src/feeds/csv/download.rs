use std::ffi::FromVecWithNulError;
use std::fs::File;
use anyhow::Result;
use time::OffsetDateTime;
use crate::feeds::Bar;
use super::YFinance;


/// https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html
/// Example url to download historial csv data: https://query1.finance.yahoo.com/v7/finance/download/TLT?period1=345479400&period2=1717257709&interval=1d&events=history&includeAdjustedClose=true
/// It works well in brower `but` met Status 429 with `reqwest` GET RESTful request.
/// After learn the source code of https://github.com/ranaroussi/yfinance.
/// I notice that I should use https://query1.finance.yahoo.com/v8/finance/chart/AAPL fetch the chart data rather than access the download url
///
///
/// https://stackoverflow.com/questions/78111453/yahoo-finance-api-file-get-contents-429-too-many-requests
/// add User Agent to solve `429` error
pub async fn download(symbol: &str) -> Result<()> {
    // TODO: change the period
    let url = format!("https://query1.finance.yahoo.com/v7/finance/download/{symbol}?period1=345479400&period2=1717257709&interval=1d&events=history&includeAdjustedClose=true");

    let client = reqwest::Client::builder()
        .user_agent("curl/7.68.0")
        .build()?;

    let response = client.get(url).send().await?;
    log::info!("response: {:?}", response);
    log::info!("Status Code: {}", response.status());
    let response_body = response.text().await?;

    // type of response_body:  alloc::string::String
    log::info!("Response body: {}", response_body);

    let finance_data: Vec<YFinance> = csv::ReaderBuilder::new()
        .from_reader(response_body.as_bytes())
        .records()
        .map(|record| {
            record.unwrap().deserialize::<YFinance>(None).unwrap()
        })
        .collect();

    let mut wtr = csv::WriterBuilder::new().from_path(format!("data/{symbol}.csv"))?;

    // set header
    wtr.write_record(&["date", "open", "high", "low", "close", "adj_close", "volume"])?;

    // set record
    for record in finance_data {
        wtr.write_record(&[
            record.date,
            record.open.to_string(),
            record.high.to_string(),
            record.low.to_string(),
            record.close.to_string(),
            record.adj_close.to_string(),
            record.volume.to_string(),
        ])?;
    }
    wtr.flush()?;

    Ok(())
}