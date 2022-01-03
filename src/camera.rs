use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use crate::primitive::{ray::Ray, vector::Vector};

/*-----------------------------------------------------------------------------------------------*/

#[derive(Clone, Copy, Debug)]
pub struct OrbitalCamera {
    position: Vector,
    focus: Vector,

    theta: f32,
    phi: f32,
    radius: f32,

    direction_perpendiculars: [Vector; 2],
    aperture: f32,

    pixel_size: f32,
}

impl OrbitalCamera {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        focus: Vector,
        radius: f32,
        aperture: f32,
    ) -> Self {
        let viewing_angle = PI / 6.0;
        let pixel_size = viewing_angle.tan() / (screen_height as f32 / 2.0);

        let mut orbital_cam = Self {
            position: Vector::new(0.0, 0.0, 0.0),
            focus,
            theta: 0.0,
            phi: 0.0,
            radius,

            direction_perpendiculars: [Vector::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 0.0)],
            aperture,

            pixel_size,
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
        self.theta += dtheta;
        self.phi += dphi;
        self.phi = self.phi.min((PI / 2.0) - 0.01).max((-PI / 2.0) + 0.01);
        self.refresh_position();
    }

    pub fn sample_pixel_ray(&self, pixel_offset: [i32; 2]) -> Ray {
        let mut rng = thread_rng();
        let radius = self.aperture * (rng.gen_range(0.0, 1.0) as f32).sqrt();
        let theta = rng.gen_range(0.0, 2.0 * PI);

        let mut random_offset = self.direction_perpendiculars[0]
            .normalized_to(theta.cos())
            .plus(&self.direction_perpendiculars[1].normalized_to(theta.sin()));
        random_offset.normalize_to(radius);

        let random_apeture_position = self.position.plus(&random_offset);
        let direction = self.focus.minus(&random_apeture_position).normalized();

        let perpendicular_x = direction.cross(&Vector::y());
        let perpendicular_y = perpendicular_x.cross(&direction);

        let offset0 = -pixel_offset[0] as f32;
        let offset1 = pixel_offset[1] as f32;
        let direction = direction
            .plus(&perpendicular_x.normalized_to(self.pixel_size * rng.gen_range(offset0 - 0.5, offset0 + 0.5)))
            .plus(&perpendicular_y.normalized_to(self.pixel_size * rng.gen_range(offset1 - 0.5, offset1 + 0.5)));
        Ray {
            origin: random_apeture_position,
            direction,
        }
    }

    fn refresh_position(&mut self) {
        let mut direction = Vector::z();
        direction.rotate_around_vector(&Vector::x(), self.phi);
        direction.rotate_around_vector(&Vector::y(), self.theta);
        direction.normalize_to(self.radius);

        self.position = self.focus.minus(&direction);

        self.direction_perpendiculars[0] = direction.cross(&Vector::y()).normalized();
        self.direction_perpendiculars[1] = self.direction_perpendiculars[0]
            .cross(&direction)
            .normalized();
    }
}
