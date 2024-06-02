pub mod fetch;
pub(crate) mod download;

#[derive(Debug, serde::Deserialize, PartialEq)]
struct YFinance {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    adj_close: f64,
    volume: i64
}