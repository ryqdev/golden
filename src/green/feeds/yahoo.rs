// use std::fs::File;
// use eframe::epaint::{Color32, Stroke};
// use egui_plot::{BoxElem, BoxPlot, BoxSpread};
//
// pub struct YahooFinanceData{
//     pub(crate) csv_file_path: String,
//     pub(crate) start_date: String,
//     pub(crate) end_date: String,
//     data: Vec<Vec<f64>>,
// }
//
// type HistoricalData = (String, Vec<f64>, usize);
//
// impl YahooFinanceData{
//     fn fetch_csv_data(symbol: &str) -> anyhow::Result<Vec<Vec<f64>>> {
//         // Date,Open,High,Low,Close,Adj Close,Volume
//         let file = File::open(format!("data/{symbol}.csv"))?;
//
//         let mut reader = csv::ReaderBuilder::new()
//             .has_headers(true)
//             .from_reader(file);
//
//         let mut finance_data = vec![];
//         for result in reader.deserialize() {
//             let record: HistoricalData = result?;
//             let low = record.1[2];
//             let open = record.1[0];
//             let close = record.1[3];
//             let high = record.1[1];
//             finance_data.push(vec![open, high, low, close])
//         }
//         Ok(finance_data)
//     }
// }
//
//
//
