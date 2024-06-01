use egui::{Stroke, Color32};
use egui_plot::{Plot, BoxPlot, BoxElem, BoxSpread, Legend, Line};
use crate::feeds::Bar;
use crate::broker::backtest::backtest::Order;
use anyhow::Result;


pub struct App {
    pub candle_data:  Vec<Bar>,
    pub cash_data: Vec<f64>,
    pub order_data:  Vec<Order>
}

fn fetch_box_data(candle_data: &Vec<Bar>) -> Result<BoxPlot> {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);

    let mut historial_data =  vec![];
    for (idx, bar) in candle_data.iter().enumerate() {
        let color = if bar.close >= bar.open {green} else {red};
        historial_data.push(BoxElem::new(idx as f64, BoxSpread::new(bar.low, bar.open, bar.open, bar.close, bar.high)).whisker_width(0.0).fill(color).stroke(Stroke::new(2.0, color)));
    }
    Ok(BoxPlot::new(historial_data).name("candle"))
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("portfolio")
            .default_width(1920f32)
            .show(ctx, |ui| {
                ui.label("Portfolio: 100_000");
                ui.label("Orders:");
                // for order in &self.order_data {
                //     ui.horizontal(|ui| {
                //         ui.label(order.size.to_owned());
                //     });
                // }
            });

        egui::Window::new("Candle")
            .fixed_pos(egui::pos2(0f32, 0f32))
            .fixed_size(egui::vec2(1920f32, 1080f32))
            .pivot(egui::Align2::CENTER_CENTER)
            .show(ctx, |ui|{
                ui.label("Candle");

                Plot::new("plot")
                    .legend(Legend::default())
                    .height(1000.0)
                    .width(2000.0)
                    .show(ui, |plot_ui| {
                        let data = fetch_box_data(&self.candle_data).expect("fetch csv data error");
                        plot_ui.box_plot(data);
                    });
            });

        egui::Window::new("Portfolio")
            .fixed_pos(egui::pos2(0f32, 1080f32))
            .fixed_size(egui::vec2(1920f32, 1080f32))
            .show(ctx, |ui|{
                Plot::new("plot")
                    .legend(Legend::default())
                    .height(1000.0)
                    .width(2000.0)
                    .show(ui, |plot_ui| {
                        let cash_line: egui_plot::PlotPoints = self.cash_data.iter()
                            .enumerate()
                            .map(|(i, value)| [i as f64, (value * 1.0) as f64])
                            .collect();
                        plot_ui.line(
                            Line::new(cash_line)
                                .color(Color32::GREEN)
                                .width(2.0)
                                .name("Total Assets")
                        );
                    });
            });
    }
}
