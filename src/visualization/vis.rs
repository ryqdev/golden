use egui::{Stroke, Color32};
use egui_plot::{Plot, BoxPlot, BoxElem, BoxSpread, Legend, Line};
use crate::feeds::Bar;
use crate::broker::backtest::backtest::Order;
use anyhow::Result;


pub struct App {
    pub candle_data:  Vec<Bar>,
    pub cash_data: Vec<f64>,
    pub net_assets_data: Vec<f64>,
    pub order_data:  Vec<Order>
}

fn fetch_box_data(candle_data: &Vec<Bar>) -> Result<BoxPlot> {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);

    Ok(BoxPlot::new(candle_data.iter()
        .enumerate()
        .map(|(idx, bar)| {
            let color = if bar.close >= bar.open {green} else {red};
            BoxElem::new(idx as f64, BoxSpread::new(bar.low, bar.open, bar.open, bar.close, bar.high)).whisker_width(0.0).fill(color).stroke(Stroke::new(2.0, color))
        }).collect()
    ).name("candle"))
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::right("portfolio")
            .show(ctx, |ui| {
                ui.label("Portfolio: 100_000_000");
                ui.label("Orders: TODO");
                // for order in &self.order_data {
                //     ui.horizontal(|ui| {
                //         ui.label(order.size.to_owned());
                //     });
                // }
            });

        egui::Window::new("Candle")
            .show(ctx, |ui|{
                ui.label("Candle");

                Plot::new("plot")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        let data = fetch_box_data(&self.candle_data).expect("fetch csv data error");
                        plot_ui.box_plot(data);
                    });
            });

        egui::Window::new("Portfolio")
            .show(ctx, |ui|{
                Plot::new("plot")
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        let cash_line: egui_plot::PlotPoints = self.cash_data.iter()
                            .enumerate()
                            .map(|(i, value)| [i as f64, value * 1.0])
                            .collect();
                        plot_ui.line(
                            Line::new(cash_line)
                                .color(Color32::GREEN)
                                .width(2.0)
                                .name("Cash Line")
                        );

                        let net_assets_data: egui_plot::PlotPoints = self.net_assets_data.iter()
                            .enumerate()
                            .map(|(i, value)| [i as f64, value * 1.0])
                            .collect();
                        plot_ui.line(
                            Line::new(net_assets_data)
                                .color(Color32::YELLOW)
                                .width(2.0)
                                .name("Net Assets")
                        );
                    });
            });
    }
}
