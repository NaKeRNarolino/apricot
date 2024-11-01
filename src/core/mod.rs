use app_wrapper::AppWrapper;
use eframe::egui;
use widget::Widget;
pub mod app_wrapper;
pub mod widget;
pub mod widgets_misc;

pub struct App<'a> {
    pub app_name: String,
    pub viewport_size: crate::misc::Vec2,
    pub widgets: Vec<&'a dyn Widget>,
}

impl<'a> App<'a> {
    pub fn run(&self) {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([self.viewport_size.x, self.viewport_size.y]),
            ..Default::default()
        };
        let _ = eframe::run_native(
            &self.app_name,
            options,
            Box::new(|cc: &eframe::CreationContext<'_>| {
                Ok(Box::new(AppWrapper::with_widgets(self.widgets.clone())))
            }),
        );
    }
}
