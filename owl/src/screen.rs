use crate::ox;

/// A helper struct to construct RGBA colours in various ways
pub struct Colour {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,
}

impl Colour {
    pub fn rgb_float(red: f32, green: f32, blue: f32) -> Self {
        Colour { red, green, blue, alpha: 1.0 }
    }
    pub fn rgba_float(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Colour { red, green, blue, alpha }
    }
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        let [red, green, blue] = [red, green, blue].map(|it| it as f32 / 255.0);
        Colour { red, green, blue, alpha: 1.0 }
    }
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        let [red, green, blue, alpha] = [red, green, blue, alpha].map(|it| it as f32 / 255.0);
        Colour { red, green, blue, alpha }
    }
}

/// Clear the colour buffer with the specified colour
pub fn clear_colour(colour: Colour) {
    ox::clear_colour(colour.red, colour.green, colour.blue, colour.alpha);
    ox::clear(ox::ClearFlags::ColourBuffer);
}
