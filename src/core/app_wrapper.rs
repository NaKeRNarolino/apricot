use super::{
    app::App,
    widget::{Widget, WidgetArray},
};

pub struct AppWrapper {
    widgets: WidgetArray,
}

impl AppWrapper {
    pub fn with_widgets(widgets: WidgetArray) -> Self {
        Self { widgets }
    }
}

impl eframe::App for AppWrapper {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            for widget in self.widgets.clone() {
                widget.into_ui(ui);
            }
        });
    }
}
