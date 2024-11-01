use core::{
    widget::{ApplyProperties, Row, Text},
    widgets_misc::CrossAxisAlignment,
};

use eframe::egui::text;
use misc::{Color, Vec2};
pub mod core;
pub mod misc;

// APRICOT ! By NaKeR Narolino

fn text_style(text: &mut Text) -> Text {
    text.with_color(Color::from_rgba(255, 240, 0, 255))
        .with_size(32.0)
}

fn main() {
    core::App {
        app_name: String::from("App Name"),
        viewport_size: Vec2 { x: 320.0, y: 240.0 },
        widgets: vec![&Row::create(vec![
            &Text::create("is a nice wrapper for egui"),
            &Text::create("Apricot").apply_properties(text_style),
        ])
        .with_cross_axis_alignment(CrossAxisAlignment::End)
        .with_reversed(true)],
    }
    .run();
}
