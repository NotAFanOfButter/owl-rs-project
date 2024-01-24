use crate::ox;

/// A helper struct to construct RGBA colours in various ways
#[derive(Clone, Copy, PartialEq)]
pub struct Colour {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

#[allow(clippy::must_use_candidate)]
impl Colour {
    pub const fn rgb_float(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue, alpha: 1.0 }
    }
    pub const fn rgba_float(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self { red, green, blue, alpha }
    }
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        let [red, green, blue] = [red, green, blue].map(|it| f32::from(it) / 255.0);
        Self { red, green, blue, alpha: 1.0 }
    }
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let [red, green, blue, alpha] = [red, green, blue, alpha].map(|it| f32::from(it) / 255.0);
        Self { red, green, blue, alpha }
    }
    pub fn greyscale(value: u8) -> Self {
        let value = f32::from(value);
        Self { red: value, green: value, blue: value, alpha: 1.0 }
    }
    pub const fn greyscale_float(value: f32) -> Self {
        Self { red: value, green: value, blue: value, alpha: 1.0 }
    }
}

/// Clear the colour buffer with the specified colour
pub fn clear_colour(colour: Colour) {
    ox::clear_colour(colour.red, colour.green, colour.blue, colour.alpha);
    ox::clear(ox::ClearFlags::ColourBuffer);
}
