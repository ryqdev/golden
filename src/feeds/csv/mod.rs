pub mod fetch;

#[derive(Debug, serde::Deserialize, PartialEq)]
pub struct YFinance {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64
}