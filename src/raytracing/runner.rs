use std::{
    sync::{Arc, RwLock},
};

use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{Event, MouseScrollDelta},
    },
    Display, Texture2d,
};


use crate::{
    camera::OrbitalCamera,
    jobs::JobRunner,
    primitive::{color::Color, vector::Vector},
    shape::Shape,
    texture::TextureGenerator,
};

use super::{scene::RaytracingScene, texture::{ColorColumnRange, IncrementalTextureHandle}};

const CAMERA_ROTATION_MULTIPLIER: f32 = 0.005;
const MAX_THREAD_COUNT: u32 = 16;

/*-----------------------------------------------------------------------------------------------*/

pub struct RaytracingRunner {
    width: u32,
    height: u32,

    mouse_pressed: bool,
    previous_mouse_position: Option<PhysicalPosition<f64>>,

    camera: OrbitalCamera,

    runner: JobRunner<ColorColumnRange>,
    scene: Arc<RwLock<RaytracingScene>>,
    texture_handle: IncrementalTextureHandle,
}

impl RaytracingRunner {
    pub fn new(width: u32, height: u32, focus: Vector) -> Self {
        let camera = OrbitalCamera::new(width, height, focus, 0.3);
        let runner = Self {
            width,
            height,
            mouse_pressed: false,
            previous_mouse_position: None,

            camera,

            runner: JobRunner::new(),
            scene: Arc::new(RwLock::new(RaytracingScene::new(5))),
            texture_handle: IncrementalTextureHandle::new(width, height, 100000),
        };

        runner
    }

    pub fn add_shape(&mut self, shape: impl Shape + 'static) {
        self.scene.write().unwrap().add_shape(Box::new(shape));
    }

    fn collect_image(&mut self) {
        for color_range in self.runner.wait_for_all_to_finish().into_iter() {
            self.texture_handle.add_color_range(color_range);
        }
    }

    fn calculate_next_image(&mut self) {
        let half_width = (self.width / 2) as i32;
        let half_height = (self.height / 2) as i32;

        let width_thread_chunk = self.width / MAX_THREAD_COUNT;

        let mut x_range: [u32; 2] = [0, 0];
        let height = self.height;
        while x_range[1] < self.width {
            x_range[1] = x_range[0] + width_thread_chunk;
            x_range[1] = self.width.min(x_range[1]);

            let x_range_to_cover = x_range.clone();
            let scene = Arc::clone(&self.scene);

            let camera = self.camera;
            self.runner.run_on_thread(move || -> ColorColumnRange {
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
}

impl TextureGenerator for RaytracingRunner {
    fn get_texture_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    fn update_texture(&mut self, display: &Display) -> Texture2d {
        self.calculate_next_image();
        self.collect_image();
        let texture = self.texture_handle.get_texture(display);

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

                            self.texture_handle.reset();
                        }

                        self.previous_mouse_position = Some(position.clone());
                    }
                }
                glium::glutin::event::WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::LineDelta(_, y) = delta {
                        self.camera.delta_zoom(*y);
                        self.texture_handle.reset();
                    }
                }
                _ => (),
            },
            _ => (),
        };
    }
}
