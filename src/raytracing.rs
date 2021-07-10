use std::{
    f32::consts::PI,
    sync::{Arc, RwLock},
};

use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{Event, MouseScrollDelta},
    },
    texture::RawImage2d,
    Display, Texture2d,
};
use rand::{thread_rng, Rng};

use crate::{
    camera::OrbitalCamera,
    jobs::JobRunner,
    primitive::{color::Color, contact::RayContact, ray::Ray, vector::Vector},
    shape::Shape,
    texture::TextureGenerator,
};

const CAMERA_ROTATION_MULTIPLIER: f32 = 0.005;
const MAX_THREAD_COUNT: u32 = 16;

/*-----------------------------------------------------------------------------------------------*/

struct ColorRange {
    starting_column: u32,
    color_columns: Vec<Vec<Color>>,
}

unsafe impl Send for ColorRange {}
unsafe impl Sync for ColorRange {}

/*-----------------------------------------------------------------------------------------------*/

struct IncrementalTextureHandle {
    width: u32,
    height: u32,
    data: Vec<Color>,
    counts: Vec<u32>,
    max_count: u32,
}

impl IncrementalTextureHandle {
    fn new(width: u32, height: u32, max_count: u32) -> Self {
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
        }
    }

    fn add_color(&mut self, x: u32, y: u32, color: &Color) {
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

    fn add_color_range(&mut self, color_range: ColorRange) {
        for (i, column) in color_range.color_columns.into_iter().enumerate() {
            self.add_color_column(i as u32 + color_range.starting_column, column);
        }
    }

    fn add_color_column(&mut self, x: u32, colors: Vec<Color>) {
        for (y, color) in colors.iter().enumerate() {
            self.add_color(x, y as u32, &color);
        }
    }

    fn reset(&mut self) {
        for i in 0..self.data.len() {
            self.data[i as usize] = Color::new(0.0, 0.0, 0.0);
            self.counts[i as usize] = 0;
        }
    }

    fn get_texture(&self, display: &Display) -> Texture2d {
        Texture2d::new(
            display,
            RawImage2d::from_raw_rgb(self.data.clone(), (self.width, self.height)),
        )
        .unwrap()
    }
}

/*-----------------------------------------------------------------------------------------------*/

struct RaytracingScene {
    camera: OrbitalCamera,
    shapes: Vec<Box<dyn Shape>>,

    max_bounce_count: u32,
}

impl RaytracingScene {
    fn new(width: u32, height: u32, focus: Vector) -> Self {
        let camera = OrbitalCamera::new(width, height, focus, 4.0);
        RaytracingScene {
            camera,
            shapes: Vec::new(),

            max_bounce_count: 4,
        }
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    fn get_pixel_color(&self, dx: i32, dy: i32) -> Color {
        let ray = self.camera.sample_pixel_ray([dx, dy]);
        self.project_ray(&ray, 0, None)
    }

    fn project_ray(
        &self,
        ray: &Ray,
        recursive_index: u32,
        shape_to_ignore: Option<usize>,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        if recursive_index < self.max_bounce_count {
            if let Some(contact) = self.find_closest_contact(&ray, shape_to_ignore) {
                let shape = &self.shapes[contact.shape_id];
                let shape_properties = shape.get_shape_properties();

                let mut shape_color_retention = 1.0;
                if let Some(shininess) = &shape_properties.shininess {
                    let mut reflection = ray
                        .direction
                        .minus(&ray.direction.project_onto(&contact.normal).times(2.0));
                    reflection.normalize();

                    let mut new_direction = Vector::from(&contact.normal); /*
                                                            .times(shininess.value.acos())
                                                            .plus(&reflection.times(shininess.value.asin()));
                                                            */
                    let max_rough_angle = shininess.roughness.asin();
                    new_direction.rotate_around_vector(
                        &new_direction.random_perpendicular(),
                        thread_rng().gen_range(0.0..max_rough_angle),
                    );

                    if new_direction.dot(&contact.normal) > 0.0 {
                        let new_ray = Ray {
                            origin: contact.position,
                            direction: new_direction,
                        };

                        let reflected_color =
                            self.project_ray(&new_ray, recursive_index + 1, Some(contact.shape_id));
                        color.r += shape_color_retention * shininess.value * reflected_color.r;
                        color.g += shape_color_retention * shininess.value * reflected_color.g;
                        color.b += shape_color_retention * shininess.value * reflected_color.b;
                    }

                    shape_color_retention *= 1.0 - shininess.value;
                }

                color.r += shape_color_retention * shape_properties.color.r;
                color.g += shape_color_retention * shape_properties.color.g;
                color.b += shape_color_retention * shape_properties.color.b;
            }
        }

        color
    }

    fn find_closest_contact(
        &self,
        ray: &Ray,
        shape_to_ignore: Option<usize>,
    ) -> Option<RayContact> {
        let mut closest_contact: Option<RayContact> = None;
        let mut closest_contact_distance = f32::MAX;
        for (shape_id, shape) in self.shapes.iter().enumerate() {
            let mut ignore_shape = false;
            if let Some(shape_id_to_ignore) = shape_to_ignore {
                ignore_shape = shape_id_to_ignore == shape_id;
            }

            if !ignore_shape {
                if let Some(mut contact) = shape.get_contact(&ray) {
                    let contact_distance = ray.origin.distance_to_sqr(&contact.position);
                    if contact_distance < closest_contact_distance {
                        contact.shape_id = shape_id;
                        closest_contact = Some(contact);
                        closest_contact_distance = contact_distance;
                    }
                }
            }
        }

        closest_contact
    }
}

pub struct RaytracingRunner {
    width: u32,
    height: u32,

    mouse_pressed: bool,
    previous_mouse_position: Option<PhysicalPosition<f64>>,

    scene: Arc<RwLock<RaytracingScene>>,
    texture_handle: IncrementalTextureHandle,
}

impl RaytracingRunner {
    pub fn new(width: u32, height: u32, focus: Vector) -> Self {
        Self {
            width,
            height,
            mouse_pressed: false,
            previous_mouse_position: None,

            scene: Arc::new(RwLock::new(RaytracingScene::new(width, height, focus))),
            texture_handle: IncrementalTextureHandle::new(width, height, 100000),
        }
    }

    pub fn add_shape(&mut self, shape: impl Shape + 'static) {
        self.scene.write().unwrap().add_shape(Box::new(shape));
    }

    fn update_internal(&mut self) {
        let mut runner = JobRunner::<ColorRange>::new();

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
            runner.run_on_thread(move || -> ColorRange {
                let mut color_range = ColorRange {
                    starting_column: x_range_to_cover[0],
                    color_columns: Vec::new(),
                };

                for x in x_range_to_cover[0]..x_range_to_cover[1] {
                    let mut color_column = Vec::<Color>::with_capacity(height as usize);
                    for y in 0..height {
                        let color = scene
                            .read()
                            .unwrap()
                            .get_pixel_color(x as i32 - half_width, y as i32 - half_height);
                        color_column.push(color);
                    }
                    color_range.color_columns.push(color_column);
                }

                color_range
            });

            x_range[0] += width_thread_chunk;
        }

        for color_range in runner.wait_for_all_to_finish().into_iter() {
            self.texture_handle.add_color_range(color_range);
        }
    }
}

impl TextureGenerator for RaytracingRunner {
    fn get_texture_size(&self) -> [u32; 2] {
        [self.width, self.height]
    }

    fn update_texture(&mut self, display: &Display) -> Texture2d {
        self.update_internal();
        self.texture_handle.get_texture(display)
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

                            self.scene.write().unwrap().camera.rotate(
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
                        self.scene.write().unwrap().camera.delta_zoom(*y);
                        self.texture_handle.reset();
                    }
                }
                _ => (),
            },
            _ => (),
        };
    }
}
