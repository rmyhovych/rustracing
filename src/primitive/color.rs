use glium::texture::{ClientFormat, PixelValue, ToClientFormat};

#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn times(self, x: f32) -> Self {
        Self::new(x * self.r, x * self.g, x * self.b)
    }

    pub fn plus(self, other: &Self) -> Self {
        Self::new(
            (self.r + other.r).min(1.0),
            (self.g + other.g).min(1.0),
            (self.b + other.b).min(1.0),
        )
    }

    pub fn filter(self, other: &Self) -> Self {
        Self::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl ToClientFormat for Color {
    fn rgb_format() -> ClientFormat {
        ClientFormat::F32F32F32
    }

    fn rgba_format() -> ClientFormat {
        ClientFormat::F32F32F32F32
    }
}

unsafe impl PixelValue for Color {
    fn get_format() -> ClientFormat {
        ClientFormat::F32F32F32
    }
}
