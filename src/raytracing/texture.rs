use glium::{texture::RawImage2d, Display, Texture2d};

use crate::primitive::color::Color;

/*-----------------------------------------------------------------------------------------------*/

pub struct ColorColumnRange {
    pub starting_column: u32,
    pub color_columns: Vec<Vec<Color>>,
}

unsafe impl Send for ColorColumnRange {}
unsafe impl Sync for ColorColumnRange {}

/*-----------------------------------------------------------------------------------------------*/

pub struct IncrementalTextureHandle {
    width: u32,
    height: u32,
    data: Vec<Color>,
    counts: Vec<u32>,
    max_count: u32,

    is_invalid: bool,
}

impl IncrementalTextureHandle {
    pub fn new(width: u32, height: u32, max_count: u32) -> Self {
        let mut data = Vec::new();
        data.resize((width * height) as usize, Color::new(0.0, 0.0, 0.0));

        let mut counts = Vec::new();
        counts.resize((width * height) as usize, 0);

        IncrementalTextureHandle {
            width,
            height,
            data,
            counts,
            max_count,

            is_invalid: true,
        }
    }

    pub fn add_color(&mut self, x: u32, y: u32, color: &Color) {
        let index = (y * self.width + x) as usize;
        let count = self.counts[index];
        if count < self.max_count {
            let mut final_color = self.data[index];
            final_color.r = (final_color.r * count as f32 + color.r) / (count + 1) as f32;
            final_color.g = (final_color.g * count as f32 + color.g) / (count + 1) as f32;
            final_color.b = (final_color.b * count as f32 + color.b) / (count + 1) as f32;
            self.data[index] = final_color;
            self.counts[index] = count + 1;
        }
    }

    pub fn add_color_range(&mut self, color_range: ColorColumnRange) {
        for (i, column) in color_range.color_columns.into_iter().enumerate() {
            self.add_color_column(i as u32 + color_range.starting_column, column);
        }
    }

    pub fn add_color_column(&mut self, x: u32, colors: Vec<Color>) {
        for (y, color) in colors.iter().enumerate() {
            self.add_color(x, y as u32, &color);
        }
    }

    pub fn invalidate(&mut self) {
        self.is_invalid = true;
    }

    pub fn get_texture(&mut self, display: &Display) -> Texture2d {
        let texture = Texture2d::new(
            display,
            RawImage2d::from_raw_rgb(self.data.clone(), (self.width, self.height)),
        )
        .unwrap();

        if self.is_invalid {
            self.reset();
            self.is_invalid = false;
        }

        texture
    }

    fn reset(&mut self) {
        for i in 0..self.data.len() {
            self.data[i as usize] = Color::new(0.0, 0.0, 0.0);
            self.counts[i as usize] = 0;
        }
    }
}
