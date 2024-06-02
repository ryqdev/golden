use std::fs;
use time::OffsetDateTime;
use crate::feeds::Bar;
use anyhow::Result;
use super::YFinance;


// https://docs.rs/csv/latest/csv/struct.Reader.html
pub fn get_bar_from_csv(symbol: &str) -> Result<Vec<Bar>> {
    // TODO: add a feature in the future: if the csv data is not exist, download it from yahoo finance
    csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader( fs::File::open(format!("data/{symbol}.csv"))?)
        .deserialize::<YFinance>()
        .map(|line| {
        let record = line?;
        Ok(Bar {
            date: OffsetDateTime::now_utc(),
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            // leave volume, wap and count blank
            volume: 0.0,
            wap: 0.0,
            count: 0,
        })
    }).collect()
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
pub async fn get_bar_from_yahoo(symbol: &str, save_csv: bool) -> Result<Vec<YFinance>> {
    // Currently, the period is from 2023/01/01:00:00:00 to 2024/01/01:00:00:00
    // TODO: add more configuration in the future
    let url = format!("https://query1.finance.yahoo.com/v7/finance/download/{symbol}?period1=1672502400&period2=1704038400&interval=1d&events=history&includeAdjustedClose=true");

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

    if save_csv {
        let mut wtr = csv::WriterBuilder::new().from_path(format!("data/{symbol}.csv"))?;

        // Set header for csv file.
        // The default header from yahoo finance with capital letter like Date, Open, High .....
        // Most scenarios need the lower case, so I set them as: date,open,high,low,close,adj_close,volume
        wtr.write_record(&["date", "open", "high", "low", "close", "adj_close", "volume"])?;

        // set record
        for record in finance_data.iter() {
            wtr.write_record(&[
                &record.date,
                &record.open.to_string(),
                &record.high.to_string(),
                &record.low.to_string(),
                &record.close.to_string(),
                &record.adj_close.to_string(),
                &record.volume.to_string(),
            ])?;
        }
        wtr.flush()?;
    }
    Ok(finance_data)
}