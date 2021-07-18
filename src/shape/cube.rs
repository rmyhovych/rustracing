use std::f32::consts::PI;

use crate::primitive::vector::Vector;

use super::{plane::PlaneShape, Shape, ShapeProperties};

pub struct CubeShape {
    properties: ShapeProperties,

    planes: Vec<PlaneShape>,
}

impl CubeShape {
    pub fn new(
        properties: ShapeProperties,
        center: Vector,
        width: f32,
        length: f32,
        height: f32,
    ) -> Self {
        let mut cube = Self {
            properties,
            planes: Vec::new(),
        };
        cube.add_planes(center, width, length, height, false);

        cube
    }

    pub fn new_inverted(
        properties: ShapeProperties,
        center: Vector,
        width: f32,
        length: f32,
        height: f32,
    ) -> Self {
        let mut cube = Self {
            properties,
            planes: Vec::new(),
        };
        cube.add_planes(center, width, length, height, true);

        cube
    }

    fn add_planes(
        &mut self,
        center: Vector,
        width: f32,
        length: f32,
        height: f32,
        is_inverted: bool,
    ) {
        self.planes = Vec::with_capacity(6);

        let multiplier = if is_inverted { -1.0 } else { 1.0 };

        // TOP
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(0.0, multiplier * height / 2.0, 0.0)),
            Vector::z(),
            0.0,
            length,
            width,
        ));

        // BOTTOM
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(0.0, -multiplier * height / 2.0, 0.0)),
            Vector::z(),
            PI,
            length,
            width,
        ));

        // RIGHT
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(multiplier * width / 2.0, 0.0, 0.0)),
            Vector::z(),
            -PI / 2.0,
            length,
            height,
        ));

        // LEFT
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(-multiplier * width / 2.0, 0.0, 0.0)),
            Vector::z(),
            PI / 2.0,
            length,
            height,
        ));

        // FRONT
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(0.0, 0.0, multiplier * length / 2.0)),
            Vector::x(),
            PI / 2.0,
            height,
            width,
        ));

        // BACK
        self.planes.push(PlaneShape::new(
            self.properties,
            center.plus(&Vector::new(0.0, 0.0, -multiplier * width / 2.0)),
            Vector::x(),
            -PI / 2.0,
            height,
            width,
        ));
    }
}

impl Shape for CubeShape {
    fn get_contact(
        &self,
        ray: &crate::primitive::ray::Ray,
    ) -> Option<crate::primitive::contact::RayContact> {
        let mut contact = None;
        for plane in &self.planes {
            contact = plane.get_contact(&ray);
            if contact.is_some() {
                break;
            }
        }

        contact
    }

    fn get_shape_properties(&self) -> &ShapeProperties {
        &self.properties
    }
}
