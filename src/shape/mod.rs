pub mod cube;
pub mod plane;
pub mod sphere;

use crate::primitive::{color::Color, contact::RayContact, ray::Ray};

#[derive(Clone, Copy)]
pub struct Shininess {
    pub value: f32,
    pub roughness: f32,
}

#[derive(Clone, Copy)]
pub struct Transparency {
    pub value: f32,
    pub density: f32,
}

#[derive(Clone, Copy)]
pub struct ShapeProperties {
    pub color: Color,
    pub shininess: Option<Shininess>,
    pub transparency: Option<Transparency>,
}

pub trait Shape: Sync + Send {
    fn get_contact(&self, ray: &Ray) -> Option<RayContact>;

    fn get_shape_properties(&self) -> &ShapeProperties;
}
