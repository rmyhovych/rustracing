pub mod cube;
mod plane;
pub mod sphere;

use crate::primitive::{color::Color, contact::RayContact, ray::Ray};

/* ------------------------------------------------------------ */

#[derive(Clone, Copy)]
pub enum ShapeType {
    Emitter,
    Reflector {
        transparency: f32,
        roughness: f32,
        density: f32,
    },
}

#[derive(Clone, Copy)]
pub struct ShapeProperties {
    pub color: Color,
    pub shape_type: ShapeType,
}

/* ------------------------------------------------------------ */
pub trait Object: Sync + Send {
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>>;

    fn get_properties(&self) -> &ShapeProperties;
}

pub trait Shape: Sync + Send {
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>>;
}

/* ------------------------------------------------------------ */

pub struct PhysicalObject<C: Shape + Sync + Send> {
    properties: ShapeProperties,
    shape: C,
}

impl<C: Shape + Sync + Send> PhysicalObject<C> {
    pub fn new(properties: ShapeProperties, shape: C) -> Self {
        Self { properties, shape }
    }
}

impl<C: Shape + Sync + Send> Object for PhysicalObject<C> {
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        self.shape.get_contact(ray)
    }

    fn get_properties(&self) -> &ShapeProperties {
        &self.properties
    }
}
