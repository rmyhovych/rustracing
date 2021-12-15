use crate::primitive::{contact::RayContact, ray::Ray, vector::Vector};

use super::Shape;

pub struct SphereShape {
    position: Vector,
    radius: f32,
}

impl SphereShape {
    pub fn new(position: Vector, radius: f32) -> Self {
        Self { position, radius }
    }
}

impl Shape for SphereShape {
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
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
            if points[1] > 0.0 {
                if points[0] > 0.0 {
                    multiplier = points[0].min(points[1])
                } else {
                    multiplier = points[1];
                }
            }

            if multiplier > 0.0 {
                let position = ray.origin.plus(&ray.direction.times(multiplier));
                let normal = position.minus(&self.position).normalized();

                Some(RayContact::new(
                    normal.times(self.radius).plus(&self.position),
                    normal,
                    ray,
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
}
