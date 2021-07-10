use glium::{glutin::event, Display, Texture2d};

pub trait TextureGenerator {
    fn get_texture_size(&self) -> [u32; 2];

    fn update_texture(&mut self, display: &Display) -> Texture2d;

    fn handle_event<T: 'static>(&mut self, event: &event::Event<T>);
}
