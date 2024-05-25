use std::collections::VecDeque;
use std::fs::File;
use eframe::epaint::{Color32, Stroke};
use egui_plot::{BoxElem, BoxPlot, BoxSpread};
use time::OffsetDateTime;

pub struct YahooFinanceData{
    pub(crate) csv_file_path: String,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
    data: Vec<Vec<f64>>,
}

type HistoricalData = (String, Vec<f64>, usize);

impl YahooFinanceData{
    fn fetch_csv_data(symbol: &str) -> anyhow::Result<Vec<Vec<f64>>> {
        // Date,Open,High,Low,Close,Adj Close,Volume
        let file = File::open(format!("data/{symbol}.csv"))?;

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut finance_data = vec![];
        for result in reader.deserialize() {
            let record: HistoricalData = result?;
            let low = record.1[2];
            let open = record.1[0];
            let close = record.1[3];
            let high = record.1[1];
            finance_data.push(vec![open, high, low, close])
        }
        Ok(finance_data)
    }
}

pub struct Bar {
    pub date: OffsetDateTime,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub wap: f64,
    pub count: i32,
}

struct BreakoutChannel {
    ticks: VecDeque<(f64, f64)>,
    size: usize,
}

impl BreakoutChannel {
    fn new(size: usize) -> BreakoutChannel {
        BreakoutChannel {
            ticks: VecDeque::with_capacity(size + 1),
            size,
        }
    }

    fn ready(&self) -> bool {
        self.ticks.len() >= self.size
    }

    fn add_bar(&mut self, bar: &Bar) {
        self.ticks.push_back((bar.high, bar.low));

        if self.ticks.len() > self.size {
            self.ticks.pop_front();
        }
    }

    fn high(&self) -> f64 {
        self.ticks.iter().map(|x| x.0).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn low(&self) -> f64 {
        self.ticks.iter().map(|x| x.1).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }
}

