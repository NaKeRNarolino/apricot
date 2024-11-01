use super::{
    widget::{Widget, WidgetArray},
    App,
};

pub struct AppWrapper<'a> {
    widgets: WidgetArray<'a>,
}

impl<'a> AppWrapper<'a> {
    pub fn with_widgets(widgets: WidgetArray<'a>) -> Self {
        Self { widgets }
    }
}

impl<'a> eframe::App for AppWrapper<'a> {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            for widget in self.widgets.clone() {
                widget.into_ui(ui);
            }
        });
    }
}
