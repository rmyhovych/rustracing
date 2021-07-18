use std::io::Seek;

use crate::primitive::{contact::RayContact, ray::Ray, vector::Vector};

use super::{Shape, ShapeProperties};

pub struct PlaneShape {
    properties: ShapeProperties,

    center: Vector,
    normal: Vector,

    length_vector: Vector,
    width_vector: Vector,

    half_length: f32,
    half_width: f32,
}

impl PlaneShape {
    pub fn new(
        properties: ShapeProperties,
        center: Vector,
        rotation_vector: Vector,
        rotation_angle: f32,
        length: f32,
        width: f32,
    ) -> Self {
        let mut normal = Vector::y();
        normal.rotate_around_vector(&rotation_vector, rotation_angle);

        let mut width_vector = Vector::x();
        width_vector.rotate_around_vector(&rotation_vector, rotation_angle);
        width_vector.normalize();

        let mut length_vector = normal.cross(&width_vector);
        length_vector.normalize();

        Self {
            properties,

            center,
            normal,

            length_vector,
            width_vector,

            half_length: length / 2.0,
            half_width: width / 2.0,
        }
    }
}

impl Shape for PlaneShape {
    /*
    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        if ray.direction.dot(&self.normal) < 0.0 {
            let perpendicular = ray.direction.project_onto(&self.normal);
            let perpendicular_distance = self.center.minus(&ray.origin).project_onto(&self.normal);
            if perpendicular_distance.dot(&perpendicular) > 0.0 {
                let distance_multiplier = perpendicular_distance.len() / perpendicular.len();

                let contact_point = ray.origin.plus(&ray.direction.times(distance_multiplier));
                let contact_from_center = self.center.minus(&contact_point);

                let from_center_length = contact_from_center.project_onto(&self.length_vector);
                if from_center_length.len() < self.half_length {
                    let from_center_width = contact_from_center.project_onto(&self.width_vector);
                    if contact_from_center.len() < self.half_width {
                        let position = self
                            .center
                            .plus(&from_center_length)
                            .plus(&from_center_width);

                        Some(RayContact::new(position, Vector::from(&self.normal), ray))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
    */

    fn get_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        if ray.direction.dot(&self.normal) < 0.0 {
            let perpendicular = ray.direction.project_onto(&self.normal);
            let perpendicular_distance = self.center.minus(&ray.origin).project_onto(&self.normal);
            if perpendicular_distance.dot(&perpendicular) > 0.0 {
                let distance_multiplier = perpendicular_distance.len() / perpendicular.len();

                let contact_point = ray.origin.plus(&ray.direction.times(distance_multiplier));
                let contact_from_center = self.center.minus(&contact_point);

                let from_center_length = contact_from_center.project_onto(&self.length_vector);
                if from_center_length.len() < self.half_length {
                    let from_center_width = contact_from_center.project_onto(&self.width_vector);
                    if from_center_width.len() < self.half_width {
                        Some(RayContact::new(
                            contact_point,
                            Vector::from(&self.normal),
                            ray,
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_shape_properties(&self) -> &ShapeProperties {
        &self.properties
    }
}
