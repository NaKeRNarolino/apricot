use std::{collections::HashMap, sync::Arc};

use eframe::egui::{self, vec2, Align, Button, Layout, RichText, ScrollArea, Style};

use crate::misc::{Color, OccupyPolicy, Orientation};

use super::widgets_misc::CrossAxisAlignment;
pub trait Widget {
    fn into_ui(&self, ui: &mut eframe::egui::Ui);
    fn build(&self) -> Arc<Self>
    where
        Self: Clone,
    {
        Arc::new(self.clone())
    }
}

pub trait ApplyProperties {
    fn apply_properties(&mut self, property_applier: fn(&mut Self) -> Self) -> Self
    where
        Self: Sized,
    {
        property_applier(self)
    }
}

pub type WidgetArray = Vec<Arc<dyn Widget>>;

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
pub struct Row {
    children: WidgetArray,
    cross_axis_alignment: CrossAxisAlignment,
    reversed: bool,
    cross_axis_occupy_policy: OccupyPolicy,
}

impl Widget for Row {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        let mut children = self.children.clone();
        if self.reversed {
            children.reverse();
        }
        ui.allocate_ui_with_layout(
            vec2(
                0.0,
                match self.cross_axis_occupy_policy {
                    OccupyPolicy::Minimal => 0.0,
                    OccupyPolicy::Full => ui.available_size().y,
                    OccupyPolicy::Custom(v) => v,
                },
            ),
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

impl Row {
    pub fn create(children: WidgetArray) -> Self {
        Self {
            children,
            cross_axis_alignment: CrossAxisAlignment::Start,
            reversed: false,
            cross_axis_occupy_policy: OccupyPolicy::Minimal,
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

    pub fn with_cross_axis_occupy_policy(&mut self, policy: OccupyPolicy) -> Self {
        self.cross_axis_occupy_policy = policy;

        self.clone()
    }
}

#[derive(Clone)]
pub struct ScrollableArea {
    child: Arc<dyn Widget>,
    orientation: Orientation,
}

impl Widget for ScrollableArea {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        match self.orientation {
            Orientation::Vertical => ScrollArea::vertical().show(ui, |ui| {
                self.child.into_ui(ui);
            }),
            Orientation::Horizontal => ScrollArea::horizontal().show(ui, |ui| {
                self.child.into_ui(ui);
            }),
        };
    }
}

impl ScrollableArea {
    pub fn create(child: Arc<dyn Widget>) -> Self {
        Self {
            child,
            orientation: Orientation::Vertical,
        }
    }

    pub fn with_orientation(&mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;

        self.clone()
    }
}

#[derive(Clone)]
pub struct Column {
    children: WidgetArray,
    cross_axis_alignment: CrossAxisAlignment,
    reversed: bool,
}

impl<'a> Widget for Column {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        let mut children = self.children.clone();
        if self.reversed {
            children.reverse();
        }
        ui.allocate_ui_with_layout(
            vec2(f32::INFINITY, 0.0),
            Layout::top_down(match self.cross_axis_alignment {
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

impl Column {
    pub fn create(children: WidgetArray) -> Self {
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

#[derive(Clone)]
pub struct FilledButton {
    text: String,
    on_clicked: fn(),
}

impl Widget for FilledButton {
    fn into_ui(&self, ui: &mut eframe::egui::Ui) {
        let button = ui.button(self.text.clone());

        if button.clicked() {
            self.on_clicked.clone()()
        }
    }
}

impl ApplyProperties for FilledButton {}

impl FilledButton {
    pub fn create(text: impl Into<String>, on_clicked: fn()) -> Self {
        Self {
            text: text.into(),
            on_clicked,
        }
    }
}
