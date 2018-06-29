use serde::ser::{Serialize, Serializer};
use palette::Srgb;
use num_traits::float::Float;

#[derive(Debug, Clone)]
pub struct Color<T = f32> where T: Float {
    inner: Srgb<T>
}
impl<T: Float> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Color<T> {
        Color {
            inner: Srgb::new(r, g, b)
        }
    }
}
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let conv = |f| f * 255.0;
        serializer.serialize_str(&format!("rgb({},{},{})",
            conv(self.inner.red),
            conv(self.inner.green),
            conv(self.inner.blue)
        ))
    }
}

pub mod name {
    use super::Color;

    pub fn white() -> Color { Color::new(1.0, 1.0, 1.0) }
    pub fn black() -> Color { Color::new(0.0, 0.0, 0.0) }
    pub fn red()   -> Color { Color::new(1.0, 0.0, 0.0) }
    pub fn blue()  -> Color { Color::new(0.0, 1.0, 0.0) }
    pub fn green() -> Color { Color::new(0.0, 0.0, 1.0) }
}
