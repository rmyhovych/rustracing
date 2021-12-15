use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use super::{ray::Ray, vector::Vector};

const WALL_SIZE: f32 = 0.001;

pub struct RayContact<'a> {
    object_id: usize,
    position_inner: Vector,
    position_outer: Vector,
    normal: Vector,

    ray: &'a Ray,
    distance_from_origin: f32,
    from_inside: bool,
}

impl<'a> RayContact<'a> {
    pub fn new(position: Vector, normal: Vector, ray: &'a Ray) -> Self {
        let wall_vector = normal.normalized_to(WALL_SIZE);
        let distance_from_origin = ray.origin.distance_to(&position);

        Self {
            object_id: 0,
            position_inner: position.minus(&wall_vector),
            position_outer: position.plus(&wall_vector),
            normal: normal.normalized(),

            ray,
            distance_from_origin,
            from_inside: ray.direction.dot(&normal) > 0.0,
        }
    }

    pub fn get_object_id(&self) -> usize {
        self.object_id
    }

    pub fn set_object_id(&mut self, object_id: usize) {
        self.object_id = object_id
    }

    pub fn get_distance_from_origin(&self) -> f32 {
        self.distance_from_origin
    }

    pub fn is_from_inside(&self) -> bool {
        self.from_inside
    }

    pub fn get_outer_reflection(&self, roughness: f32) -> Ray {
        self.get_random_outer_reflection()
    }

    pub fn get_refraction(&self, index_incident: f32, index_refracted: f32) -> Ray {
        let actual_normal = self.normal.times(if self.from_inside { 1.0 } else { -1.0 });
        let angle_incident = self.ray.direction.angle_between(&actual_normal);

        let refracted_sin = (index_incident / index_refracted) * angle_incident.sin();
        if refracted_sin < 1.0 && refracted_sin > -1.0 {
            let angle_refracted = refracted_sin.asin();

            let mut new_direction = Vector::from(&self.ray.direction);
            if angle_refracted != angle_incident {
                let angle_diff = angle_refracted - angle_incident;
                let rotation_axis = actual_normal.cross(&new_direction);
                new_direction.rotate_around_vector(&rotation_axis, angle_diff);
            }

            Ray {
                origin: if self.from_inside {
                    self.position_outer
                } else {
                    self.position_inner
                },
                direction: new_direction,
            }
        } else {
            self.get_mirror_reflection()
        }
    }

    fn get_mirror_reflection(&self) -> Ray {
        let onto_normal = self.ray.direction.project_onto(&self.normal);
        let direction = self.ray.direction.minus(&onto_normal.times(2.0));

        Ray {
            origin: if self.from_inside {
                self.position_inner
            } else {
                self.position_outer
            },
            direction,
        }
    }

    fn get_random_outer_reflection(&self) -> Ray {
        let random_perpendicular = self.normal.random_perpendicular();
        let mut direction = Vector::from(&self.normal);
        direction
            .rotate_around_vector(&random_perpendicular, thread_rng().gen_range(0.0..PI / 2.0));

        Ray {
            origin: self.position_outer.clone(),
            direction,
        }
    }
}
