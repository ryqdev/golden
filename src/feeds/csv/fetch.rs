use std::fs::File;
use time::OffsetDateTime;
use crate::feeds::Bar;
use anyhow::Result;
use super::YFinance;




// https://docs.rs/csv/latest/csv/struct.Reader.html
pub fn get_bar_from_csv(symbol: &str) -> Result<Vec<Bar>> {
    csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader( File::open(format!("data/{symbol}.csv"))?)
        .deserialize::<YFinance>()
        .map(|line| {
        let record = line?;
        Ok(Bar {
            date: OffsetDateTime::now_utc(),
            open: record.open,
            high: record.high,
            low: record.low,
            close: record.close,
            volume: 0.0,
            wap: 0.0,
            count: 0,
        })
    }).collect()
}
