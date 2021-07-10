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
