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

fn candlestick_chart(ui: &mut egui::Ui) {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);
    let data = BoxPlot::new(vec![
        BoxElem::new(0.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(0.5, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.5, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
    ]);

    let plot = Plot::new("candlestick chart")
        .view_aspect(2.0);

    plot.show(ui, |plot_ui| {
        plot_ui.box_plot(data);
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("value = {}", self.value));
        });
        egui::Window::new("My Window").show(ctx, |ui| {
            candlestick_chart(ui);
        });
    }
}
