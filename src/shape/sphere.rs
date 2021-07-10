use crate::primitive::{contact::RayContact, ray::Ray, vector::Vector};

use super::{Shape, ShapeProperties};

pub struct SphereShape {
    shape_properties: ShapeProperties,

    position: Vector,
    radius: f32,
}

impl SphereShape {
    pub fn new(shape_properties: ShapeProperties, position: Vector, radius: f32) -> Self {
        Self {
            shape_properties,
            position,
            radius,
        }
    }
}

impl Shape for SphereShape {
    fn get_contact(&self, ray: &Ray) -> Option<RayContact> {
        let a = ray.direction.len_sqr();
        let b = 2.0
            * ray
                .direction
                .multiply(&ray.origin.minus(&self.position))
                .sum();
        let c = ray.origin.minus(&self.position).len_sqr() - (self.radius * self.radius);

        let inside_sqrt = b * b - 4.0 * a * c;

        if inside_sqrt > 0.0 {
            let sqrt = inside_sqrt.sqrt();
            let points = [(-b + sqrt) / (2.0 * a), (-b - sqrt) / (2.0 * a)];
            let mut multiplier = points[0];
            if points[1] >= 0.0 {
                if points[0] >= 0.0 {
                    multiplier = points[0].min(points[1])
                } else {
                    multiplier = points[1];
                }
            }

            if multiplier >= 0.0 {
                let position = ray.origin.plus(&ray.direction.times(multiplier));
                let mut normal = position.minus(&self.position);
                normal.normalize();

                Some(RayContact::new(position, normal))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }
}
