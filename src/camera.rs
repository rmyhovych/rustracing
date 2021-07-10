use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use crate::primitive::{ray::Ray, vector::Vector};

pub struct Camera {
    position: Vector,
    theta: f32,
    phi: f32,

    pixel_angle: f32,
}

impl Camera {
    pub fn new(
        _screen_width: u32,
        screen_height: u32,
        position: Vector,
        theta: f32,
        phi: f32,
    ) -> Self {
        let viewing_angle = PI / 2.0;
        let pixel_angle = viewing_angle / screen_height as f32;
        Self {
            position,
            theta,
            phi,
            pixel_angle,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.position.x += x;
        self.position.y += y;
        self.position.z += z;
    }

    pub fn rotate(&mut self, dtheta: f32, dphi: f32) {
        self.theta += dtheta;
        self.phi += dphi;
        self.phi = self.phi.min((PI / 2.0) - 0.01).max((-PI / 2.0) + 0.01);
    }

    pub fn sample_pixel_ray(&self, pixel_offset: [i32; 2]) -> Ray {
        let mut ray = Ray {
            origin: Vector::from(&self.position),
            direction: Vector::z(),
        };

        let mut rng = thread_rng();
        ray.direction.x += self.pixel_angle * pixel_offset[0] as f32
            + rng.gen_range((-self.pixel_angle / 2.0)..(self.pixel_angle / 2.0));
        ray.direction.y += self.pixel_angle * pixel_offset[1] as f32
            + rng.gen_range((-self.pixel_angle / 2.0)..(self.pixel_angle / 2.0));

        ray.direction
            .rotate_around_vector(&Vector::x(), self.phi);
        ray.direction
            .rotate_around_vector(&Vector::y(), self.theta);

        ray
    }
}

/*-----------------------------------------------------------------------------------------------*/

pub struct OrbitalCamera {
    camera: Camera,
    focus: Vector,
    radius: f32,
}

impl OrbitalCamera {
    pub fn new(screen_width: u32, screen_height: u32, focus: Vector, radius: f32) -> Self {
        let mut orbital_cam = Self {
            camera: Camera::new(
                screen_width,
                screen_height,
                Vector::new(0.0, 0.0, 0.0),
                0.0,
                0.0,
            ),
            focus,
            radius,
        };

        orbital_cam.refresh_position();
        orbital_cam
    }

    pub fn translate(&mut self, x: f32, y: f32, _z: f32) {
        self.focus.x += x;
        self.focus.y += y;
        self.focus.z += x;
        self.refresh_position();
    }

    pub fn delta_zoom(&mut self, delta: f32) {
        if delta < 0.0 {
            self.radius *= 1.1;
        } else if delta > 0.0 {
            self.radius *= 0.9;
        }

        self.refresh_position();
    }

    pub fn rotate(&mut self, dtheta: f32, dphi: f32) {
        self.camera.rotate(dtheta, dphi);
        self.refresh_position();
    }

    pub fn sample_pixel_ray(&self, pixel_offset: [i32; 2]) -> Ray {
        self.camera.sample_pixel_ray(pixel_offset)
    }

    fn refresh_position(&mut self) {
        let mut direction = Vector::z();
        direction.rotate_around_vector(&Vector::x(), self.camera.phi);
        direction.rotate_around_vector(&Vector::y(), self.camera.theta);
        direction.normalize_to(self.radius);

        self.camera.position = self.focus.minus(&direction);
    }
}
