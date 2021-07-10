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
        // TODO

        let mut planes: Vec<PlaneShape> = Vec::with_capacity(6);

        Self { properties, planes }
    }
}

impl Shape for CubeShape {
    fn get_contact(
        &self,
        ray: &crate::primitive::ray::Ray,
    ) -> Option<crate::primitive::contact::RayContact> {
        None
    }

    fn get_shape_properties(&self) -> &ShapeProperties {
        &self.properties
    }
}
