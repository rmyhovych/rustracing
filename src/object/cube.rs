use std::f32::consts::PI;

use crate::primitive::{contact::RayContact, ray::Ray, vector::Vector};

use super::{plane::PlaneShape, Shape};

pub struct CubeShape {
    planes: Vec<PlaneShape>,
}

impl CubeShape {
    pub fn new(center: Vector, width: f32, length: f32, height: f32) -> Self {
        Self {
            planes: Vec::with_capacity(6),
        }
        .add_planes(center, width, length, height, false)
    }

    pub fn new_inverted(center: Vector, width: f32, length: f32, height: f32) -> Self {
        Self {
            planes: Vec::with_capacity(6),
        }
        .add_planes(center, width, length, height, true)
    }

    fn add_planes(
        mut self,
        center: Vector,
        width: f32,
        length: f32,
        height: f32,
        is_inverted: bool,
    ) -> Self {
        self.planes = Vec::with_capacity(6);

        let multiplier = if is_inverted { -1.0 } else { 1.0 };

        // TOP
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(0.0, multiplier * height / 2.0, 0.0)),
            Vector::z(),
            0.0,
            length,
            width,
        ));

        // BOTTOM
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(0.0, -multiplier * height / 2.0, 0.0)),
            Vector::z(),
            PI,
            length,
            width,
        ));

        // RIGHT
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(multiplier * width / 2.0, 0.0, 0.0)),
            Vector::z(),
            -PI / 2.0,
            length,
            height,
        ));

        // LEFT
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(-multiplier * width / 2.0, 0.0, 0.0)),
            Vector::z(),
            PI / 2.0,
            length,
            height,
        ));

        // FRONT
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(0.0, 0.0, multiplier * length / 2.0)),
            Vector::x(),
            PI / 2.0,
            height,
            width,
        ));

        // BACK
        self.planes.push(PlaneShape::new(
            center.plus(&Vector::new(0.0, 0.0, -multiplier * width / 2.0)),
            Vector::x(),
            -PI / 2.0,
            height,
            width,
        ));

        self
    }
}

impl Shape for CubeShape {
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        let mut contact = None;
        for plane in &self.planes {
            contact = plane.get_contact(&ray);
            if contact.is_some() {
                break;
            }
        }

        contact
    }
}
