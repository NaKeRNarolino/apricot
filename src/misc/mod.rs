use std::cmp::{max, min};

#[derive(Clone, Copy)]
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

    /// An HSV color, parsed from `hue` parameter(0-360 are the expected values), `saturation` parameter(0.0-1.0) and `value` parameter(0.0-1.0)
    pub fn from_hsva(hue: u16, saturation: f32, value: f32, alpha: u8) -> Self {
        let fun = |n| {
            let k = ((n as f64 + (hue as f64 / 60.0)) % 6.0).floor() as i32;
            value as f64 - value as f64 * saturation as f64 * max(0, min(min(k, 4 - k), 1)) as f64
        };

        let r = (fun(5) * 255.0).floor() as u8;
        let g = (fun(3) * 255.0).floor() as u8;
        let b = (fun(1) * 255.0).floor() as u8;
        
        Self {
            red: r,
            green: g,
            blue: b,
            alpha
        }
    }
}

// impl Into<Color32> for Color {
//     fn into(self) -> Color32 {
//         Color32::from_rgba_unmultiplied(self.red, self.green, self.blue, self.alpha)
//     }
// }

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
