use eframe::egui::{self, vec2, Align, Layout, RichText, Style};

use crate::misc::Color;

use super::widgets_misc::CrossAxisAlignment;
pub trait Widget {
    fn into_ui(&self, ui: &mut eframe::egui::Ui);
}

pub trait ApplyProperties {
    fn apply_properties(&mut self, property_applier: fn(&mut Self) -> Self) -> Self
    where
        Self: Sized,
    {
        property_applier(self)
    }
}

pub type WidgetArray<'a> = Vec<&'a dyn Widget>;

#[derive(Clone)]
pub struct Text {
    text: String,
    style: TextStyle,
}

impl Widget for Text {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        ui.label(
            RichText::new(self.text.clone())
                .color(self.style.color.clone())
                .size(self.style.size.clone()),
        );
    }
}

impl<'a> Text {
    pub fn create(text: impl Into<String>) -> Self {
        let ret: Text = Self {
            text: text.into(),
            style: TextStyle::default(),
        };

        ret
    }

    pub fn with_color(&mut self, color: Color) -> Self {
        self.style.color = color;

        self.clone()
    }

    pub fn with_size(&mut self, size: impl Into<f32>) -> Self {
        self.style.size = size.into();

        self.clone()
    }
}

impl ApplyProperties for Text {}

#[derive(Clone)]
pub struct TextStyle {
    pub color: Color,
    pub size: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            color: Color::from_rgba(255, 255, 255, 255),
            size: 16.0,
        }
    }
}

#[derive(Clone)]
pub struct Row<'a> {
    children: WidgetArray<'a>,
    cross_axis_alignment: CrossAxisAlignment,
    reversed: bool,
}

impl<'a> Widget for Row<'a> {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        let mut children = self.children.clone();
        if self.reversed {
            children.reverse();
        }
        ui.allocate_ui_with_layout(
            vec2(0.0, 0.0),
            Layout::left_to_right(match self.cross_axis_alignment {
                CrossAxisAlignment::Start => Align::TOP,
                CrossAxisAlignment::End => Align::BOTTOM,
                CrossAxisAlignment::Center => Align::Center,
            }),
            |ui| {
                for child in children {
                    child.into_ui(ui);
                }
            },
        );
    }
}

impl<'a> Row<'a> {
    pub fn create(children: WidgetArray<'a>) -> Self {
        Self {
            children,
            cross_axis_alignment: CrossAxisAlignment::Start,
            reversed: false,
        }
    }

    pub fn with_cross_axis_alignment(&mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;

        self.clone()
    }

    pub fn with_reversed(&mut self, reversed: bool) -> Self {
        self.reversed = reversed;

        self.clone()
    }
}
