use std::sync::Arc;
use winit::window::Window;
use crate::core::inner::InnerEngine;
use crate::misc::Vec2;

// use super::app_wrapper::AppWrapper;
use super::context::Context;
use super::navigation::Navigator;
// use super::screen::{ScreenArray, ScreensProvider};
// use super::widget::WidgetArray;

pub struct App {
    app_name: String,
    viewport_size: crate::misc::Vec2,
    inner_engine: InnerEngine
}

impl App {
    pub fn run(self) {
        self.inner_engine.run();
    }

    pub async fn create(
        app_name: impl Into<String> + Clone,
        viewport_size: Vec2,
        // screens: Arc<ScreensProvider>,
        // pages: Vec<P>,
        // home: P,
    ) -> Self
    {
        Self {
            app_name: app_name.clone().into(),
            viewport_size,
            inner_engine: InnerEngine::new(
                viewport_size,
                app_name.into()
            ).await
        }
    }
}
