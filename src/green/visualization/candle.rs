use std::{
    fs::File
};

use egui::{
    Stroke,
    Color32,
};
use egui_plot::{
    Plot,
    BoxPlot,
    BoxElem,
    BoxSpread,
};

pub struct App {
    value: f64,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            value: 42.0,
        }
    }
}

type HistoricalData = (String, Vec<f64>);

fn fetch_csv_data(symbol: &str) -> anyhow::Result<BoxPlot> {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);

    // Date,Open,High,Low,Close,Adj Close,Volume
    let file = File::open(format!("data/{symbol}.csv"))?;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(file);

    let mut historical_data = vec![];

    let mut idx = 0.0;
    for result in reader.deserialize() {
        let record: HistoricalData = result?;
        let low = record.1[2];
        let open = record.1[0];
        let close = record.1[3];
        let high = record.1[1];
        let color = if close >= open {green} else {red};
        historical_data.push(
            BoxElem::new(idx, BoxSpread::new(low, open, open,close , high)).whisker_width(0.0).fill(color).stroke(Stroke::new(2.0, color)),
        );
        idx += 1.0
    }

    Ok(BoxPlot::new(historical_data))
}

fn candlestick_chart(ui: &mut egui::Ui, symbol: &str) -> anyhow::Result<()> {
    let data = fetch_csv_data(symbol)?;

    let plot = Plot::new("candlestick chart")
        .view_aspect(2.0);

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
    Ok(())
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("value = {}", self.value));
        });
        egui::Window::new("My Window").show(ctx, |ui| {
            candlestick_chart(ui, "SPY").expect("render error");
        });
    }
}
