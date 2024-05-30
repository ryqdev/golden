use egui::{Stroke, Color32};
use egui_plot::{Plot, BoxPlot, BoxElem, BoxSpread, Legend, Line};

use crate::green::green::{
    Order,
    Bar
};

#[derive(Default)]
pub struct App<'a> {
    // impl Iterator<Item = ...> can be used in scenarios where the size of object is known and fixed
    // while Box<dyn Interator<Item = ...> is used otherwise
    pub(crate) candle_data: &'a dyn Iterator<Item=Bar>,
    pub cash_data: &'a Vec<f64>,
    pub net_asset_data: &'a Vec<f64>,
    pub order_data: &'a Vec<Order>
}

fn fetch_box_data(candle_data: Vec<Vec<f64>>) -> anyhow::Result<BoxPlot> {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);

    let mut historical_data = vec![];
    let mut idx = 0.0;

    for record in candle_data {
        let low = record[2];
        let open = record[0];
        let close = record[3];
        let high = record[1];
        let color = if close >= open {green} else {red};
        historical_data.push(
            BoxElem::new(idx, BoxSpread::new(low, open, open,close , high)).whisker_width(0.0).fill(color).stroke(Stroke::new(2.0, color)),
        );
        idx += 1.0
    }
    Ok(BoxPlot::new(historical_data).name("candle"))
}


impl eframe::App for App<'_> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu").show(ctx, |ui| {
            ctx.request_repaint_after(std::time::Duration::from_millis(200));
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Trading", |ui| {
                    if ui.button("Equities").clicked() {
                        println!("> Equities.");
                    }
                    if ui.button("Forex").clicked() {
                        println!("> Forex.");
                    }
                });
                ui.menu_button("Portfolios", |ui| {
                    if ui.button("Archimedes I").clicked() {
                        println!("> Archimedes I.");
                    }
                });
                ui.menu_button("Risk Management", |ui| {
                    ui.menu_button("Value-at-Risk", |ui| {
                        if ui.button("Parametric").clicked() {
                            println!("> Parametric.");
                        }
                        if ui.button("Historical").clicked() {
                            println!("> Historical.");
                        }
                        if ui.button("Monte Carlo Simulation").clicked() {
                            println!("> Monte Carlo Simulation.");
                        }
                    });
                });
            });
        });

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
                        let data = fetch_box_data(self.candle_data.clone()).expect("fetch csv data error");
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
                        let cash_line: egui_plot::PlotPoints = self.cash_data.clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, value)| [i as f64, (value * 1.0) as f64])
                            .collect();
                        plot_ui.line(
                            Line::new(cash_line)
                                .color(Color32::GREEN)
                                .width(2.0)
                                .name("Total Assets")
                        );

                        let net_asset_line: egui_plot::PlotPoints = self.net_asset_data.clone()
                            .into_iter()
                            .enumerate()
                            .map(|(i, value)| [i as f64, (value * 1.0) as f64])
                            .collect();
                        plot_ui.line(
                            Line::new(net_asset_line)
                                .color(Color32::BLUE)
                                .width(2.0)
                                .name("Net Assets")
                        );
                    });
            });
    }
}
