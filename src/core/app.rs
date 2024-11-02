use crate::misc::Vec2;

use super::app_wrapper::AppWrapper;
use super::widget::WidgetArray;
use eframe::egui::{self, widgets, Widget};

pub struct App {
    app_name: String,
    viewport_size: crate::misc::Vec2,
    widgets: WidgetArray,
}

impl App {
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

    pub fn create(app_name: impl Into<String>, viewport_size: Vec2, widgets: WidgetArray) -> Self {
        Self {
            app_name: app_name.into(),
            viewport_size,
            widgets,
        }
    }
}
