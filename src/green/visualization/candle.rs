use std::{
    fs::File
};
use eframe::emath::Vec2;

use egui::{Stroke, Color32, DragValue, Event};
use egui_plot::{Plot, BoxPlot, BoxElem, BoxSpread, Legend, PlotPoints, Line};


#[derive(Default)]
pub struct App {
    value: f64,
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
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

        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.checkbox(&mut self.lock_x, "Lock x axis").on_hover_text("Check to keep the X axis fixed, i.e., pan and zoom will only affect the Y axis");
            ui.checkbox(&mut self.lock_y, "Lock y axis").on_hover_text("Check to keep the Y axis fixed, i.e., pan and zoom will only affect the X axis");
            ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot");
            ui.checkbox(&mut self.shift_to_horizontal, "Shift for horizontal scroll").on_hover_text("If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.zoom_speed)
                        .clamp_range(0.1..=2.0)
                        .speed(0.1),
                );
                ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
            });
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.scroll_speed)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1),
                );
                ui.label("Scroll speed").on_hover_text("How fast to pan with the mouse wheel");
            });
        });

        egui::SidePanel::right("portfolio").show(ctx, |ui| {
            ui.label("Portfolio: 100000");
            ui.label("Orders:");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let (scroll, pointer_down, modifiers) = ui.input(|i| {
                let scroll = i.events.iter().find_map(|e| match e {
                    Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                });
                (scroll, i.pointer.primary_down(), i.modifiers)
            });

            ui.label("This example shows how to use raw input events to implement different plot controls than the ones egui provides by default, e.g., default to zooming instead of panning when the Ctrl key is not pressed, or controlling much it zooms with each mouse wheel step.");

            Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    if let Some(mut scroll) = scroll {
                        if modifiers.ctrl == self.ctrl_to_zoom {
                            scroll = Vec2::splat(scroll.x + scroll.y);
                            let mut zoom_factor = Vec2::from([
                                (scroll.x * self.zoom_speed / 10.0).exp(),
                                (scroll.y * self.zoom_speed / 10.0).exp(),
                            ]);
                            if self.lock_x {
                                zoom_factor.x = 1.0;
                            }
                            if self.lock_y {
                                zoom_factor.y = 1.0;
                            }
                            plot_ui.zoom_bounds_around_hovered(zoom_factor);
                        } else {
                            if modifiers.shift == self.shift_to_horizontal {
                                scroll = Vec2::new(scroll.y, scroll.x);
                            }
                            if self.lock_x {
                                scroll.x = 0.0;
                            }
                            if self.lock_y {
                                scroll.y = 0.0;
                            }
                            let delta_pos = self.scroll_speed * scroll;
                            plot_ui.translate_bounds(delta_pos);
                        }
                    }
                    if plot_ui.response().hovered() && pointer_down {
                        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                        if self.lock_x {
                            pointer_translate.x = 0.0;
                        }
                        if self.lock_y {
                            pointer_translate.y = 0.0;
                        }
                        plot_ui.translate_bounds(pointer_translate);
                    }
                    let data = fetch_csv_data("SPY").expect("fetch csv data error");
                    plot_ui.box_plot(data);
                    let sine_points = PlotPoints::from_explicit_callback(|x| x * 0.01, .., 5000);
                    plot_ui.line(Line::new(sine_points).name("Line"));
                });
        });
    }
}
