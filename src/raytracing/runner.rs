use std::sync::{Arc, RwLock};

use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{Event, MouseScrollDelta},
    },
    Display, Texture2d,
};
use threadpool::ThreadPool;

use crate::{
    camera::OrbitalCamera,
    object::{Object, PhysicalObject, Shape, ShapeProperties},
    primitive::{color::Color, vector::Vector},
    texture::TextureGenerator,
};

use super::{
    scene::RaytracingScene,
    texture::{ColorColumnRange, IncrementalTextureHandle},
};

const CAMERA_ROTATION_MULTIPLIER: f32 = 0.005;

/*-----------------------------------------------------------------------------------------------*/

pub struct RaytracingRunner {
    width: u32,
    height: u32,

    mouse_pressed: bool,
    previous_mouse_position: Option<PhysicalPosition<f64>>,

    camera: OrbitalCamera,

    work_pool: ThreadPool<ColorColumnRange>,
    scene: Arc<RwLock<RaytracingScene>>,
    texture_handle: IncrementalTextureHandle,
}

impl RaytracingRunner {
    pub fn new(width: u32, height: u32, focus: Vector) -> Self {
        let camera = OrbitalCamera::new(width, height, focus, 1.0);
        Self {
            width,
            height,
            mouse_pressed: false,
            previous_mouse_position: None,

            camera,

            work_pool: ThreadPool::new(10),
            scene: Arc::new(RwLock::new(RaytracingScene::new(6))),
            texture_handle: IncrementalTextureHandle::new(width, height, 100000),
        }
    }

    pub fn add_object(&mut self, properties: ShapeProperties, shape: impl Shape + 'static) {
        self.scene
            .write()
            .unwrap()
            .add_object(PhysicalObject::new(properties, shape));
    }

    fn collect_image(&mut self) {
        for color_range in self.work_pool.collect_results() {
            self.texture_handle.add_color_range(color_range);
        }
    }

    fn start_calculating_next_image(&mut self) {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;

        let width_thread_chunk = 1;

        let mut x_range: [u32; 2] = [0, 0];
        let height = self.height;
        while x_range[1] < self.width {
            x_range[1] = self.width.min(x_range[0] + width_thread_chunk);

            let x_range_to_cover = x_range.clone();
            let scene = Arc::clone(&self.scene);

            let camera = self.camera;
            self.work_pool.run(move || -> ColorColumnRange {
                let mut color_range = ColorColumnRange {
                    starting_column: x_range_to_cover[0],
                    color_columns: Vec::new(),
                };

                for x in x_range_to_cover[0]..x_range_to_cover[1] {
                    let mut color_column = Vec::<Color>::with_capacity(height as usize);
                    for y in 0..height {
                        let ray = camera
                            .sample_pixel_ray([x as i32 - half_width, y as i32 - half_height]);

                        let color = scene.read().unwrap().get_pixel_color(ray);
                        color_column.push(color);
                    }
                    color_range.color_columns.push(color_column);
                }

                color_range
            });

            x_range[0] += width_thread_chunk;
        }
    }

    fn invalidate_image(&mut self) {
        self.texture_handle.invalidate();
    }
}

impl TextureGenerator for RaytracingRunner {
    fn get_texture_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    fn update_texture(&mut self, display: &Display) -> Texture2d {
        self.collect_image();
        let texture = self.texture_handle.get_texture(display);
        self.start_calculating_next_image();

        texture
    }

    fn handle_event<T: 'static>(&mut self, main_event: &glium::glutin::event::Event<T>) {
        match main_event {
            Event::WindowEvent { event, .. } => match event {
                glium::glutin::event::WindowEvent::MouseInput { state, .. } => {
                    match state {
                        glium::glutin::event::ElementState::Pressed => self.mouse_pressed = true,
                        glium::glutin::event::ElementState::Released => {
                            self.mouse_pressed = false;
                            self.previous_mouse_position = None
                        }
                    };
                }
                glium::glutin::event::WindowEvent::CursorMoved { position, .. } => {
                    if self.mouse_pressed {
                        if let Some(prev_position) = self.previous_mouse_position {
                            let offset =
                                [position.x - prev_position.x, position.y - prev_position.y];

                            self.camera.rotate(
                                CAMERA_ROTATION_MULTIPLIER * offset[0] as f32,
                                CAMERA_ROTATION_MULTIPLIER * offset[1] as f32,
                            );

                            self.invalidate_image();
                        }

                        self.previous_mouse_position = Some(position.clone());
                    }
                }
                glium::glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                    let y = match delta {
                        MouseScrollDelta::LineDelta(_, y) => *y,
                        MouseScrollDelta::PixelDelta(position) => 0.1 * (position.y as f32),
                    };

                    self.camera.delta_zoom(y);
                    self.invalidate_image();
                }
                _ => (),
            },
            _ => (),
        };
    }

    fn stop(&mut self) {
        self.work_pool.stop_threads();
    }
}
