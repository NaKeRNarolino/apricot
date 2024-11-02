use eframe::egui::Color32;

#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
            alpha: 255,
        }
    }
}

impl Into<Color32> for Color {
    fn into(self) -> Color32 {
        Color32::from_rgba_unmultiplied(self.red, self.green, self.blue, self.alpha)
    }
}

#[derive(Clone, PartialEq)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Clone, PartialEq)]
pub enum OccupyPolicy {
    Minimal,
    Full,
    Custom(f32),
}
