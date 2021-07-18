use std::{
    f32::consts::PI,
    sync::{Arc, RwLock},
};

use glium::{
    glutin::{
        dpi::PhysicalPosition,
        event::{Event, MouseScrollDelta},
    },
    texture::RawImage2d,
    Display, Texture2d,
};
use rand::{thread_rng, Rng};

use crate::{
    camera::OrbitalCamera,
    jobs::JobRunner,
    primitive::{color::Color, contact::RayContact, ray::Ray, vector::Vector},
    shape::Shape,
    texture::TextureGenerator,
};

fn get_refracted_angle_delta(
    incident_angle: f32,
    incident_index: f32,
    refracted_index: f32,
) -> f32 {
    let refracted_angle_sin = (incident_index / refracted_index) * incident_angle.sin();
    if refracted_angle_sin < 1.0 {
        incident_angle - refracted_angle_sin.asin()
    } else {
        -2.0 * ((PI / 2.0) - incident_angle)
    }
}

pub struct RaytracingScene {
    shapes: Vec<Box<dyn Shape>>,

    max_bounce_count: u32,
}

impl RaytracingScene {
    pub fn new(max_bounce_count: u32) -> Self {
        Self {
            shapes: Vec::new(),
            max_bounce_count,
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn get_pixel_color(&self, ray: Ray) -> Color {
        self.project_ray(&ray, 0, None)
    }

    pub fn project_ray(
        &self,
        ray: &Ray,
        recursive_index: u32,
        shape_to_ignore: Option<usize>,
    ) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        if recursive_index < self.max_bounce_count {
            if let Some(contact) = self.find_closest_contact(&ray, shape_to_ignore) {
                let shape = &self.shapes[contact.shape_id];
                let shape_properties = shape.get_shape_properties();

                let from_inside = ray.direction.dot(&contact.normal) > 0.0;

                let mut shape_color_retention = 1.0;
                if !from_inside {
                    if let Some(shininess) = &shape_properties.shininess {
                        let mut reflection = ray
                            .direction
                            .minus(&ray.direction.project_onto(&contact.normal).times(2.0));
                        reflection.normalize();

                        let mut new_direction = Vector::from(&contact.normal); /*
                                                                               .times(shininess.value.acos())
                                                                               .plus(&reflection.times(shininess.value.asin()));
                                                                               */
                        let max_rough_angle = shininess.roughness.asin();
                        new_direction.rotate_around_vector(
                            &new_direction.random_perpendicular(),
                            thread_rng().gen_range(0.0..max_rough_angle),
                        );

                        if new_direction.dot(&contact.normal) > 0.0 {
                            let new_ray = Ray {
                                origin: contact.position,
                                direction: new_direction,
                            };

                            let reflected_color = self.project_ray(
                                &new_ray,
                                recursive_index + 1,
                                Some(contact.shape_id),
                            );
                            color.r += shape_color_retention * shininess.value * reflected_color.r;
                            color.g += shape_color_retention * shininess.value * reflected_color.g;
                            color.b += shape_color_retention * shininess.value * reflected_color.b;
                        }

                        shape_color_retention *= 1.0 - shininess.value;
                    }
                }

                if let Some(transparency) = &shape_properties.transparency {
                    let contact_normal = contact.normal.times(if from_inside { -1.0 } else { 1.0 });
                    let incident_angle = PI - ray.direction.angle_between(&contact_normal);
                    let delta_angle = if from_inside {
                        get_refracted_angle_delta(incident_angle, transparency.density, 1.0)
                    } else {
                        get_refracted_angle_delta(incident_angle, 1.0, transparency.density)
                    };

                    let rotation_axis = ray.direction.cross(&contact.normal);
                    let mut new_direction = Vector::from(&ray.direction);

                    new_direction.rotate_around_vector(&rotation_axis, delta_angle);

                    let new_ray = Ray {
                        origin: contact.position,
                        direction: new_direction,
                    };

                    let reflected_color = self.project_ray(
                        &new_ray,
                        recursive_index + 1,
                        if from_inside {
                            Some(contact.shape_id)
                        } else {
                            None
                        },
                    );

                    color.r += shape_color_retention * transparency.value * reflected_color.r;
                    color.g += shape_color_retention * transparency.value * reflected_color.g;
                    color.b += shape_color_retention * transparency.value * reflected_color.b;

                    shape_color_retention *= 1.0 - transparency.value;
                }

                color.r += shape_color_retention * shape_properties.color.r;
                color.g += shape_color_retention * shape_properties.color.g;
                color.b += shape_color_retention * shape_properties.color.b;
            }
        }

        color
    }

    fn find_closest_contact(
        &self,
        ray: &Ray,
        shape_to_ignore: Option<usize>,
    ) -> Option<RayContact> {
        let mut closest_contact: Option<RayContact> = None;
        let mut closest_contact_distance = f32::MAX;
        for (shape_id, shape) in self.shapes.iter().enumerate() {
            let mut ignore_shape = false;
            if let Some(shape_id_to_ignore) = shape_to_ignore {
                ignore_shape = shape_id_to_ignore == shape_id;
            }

            if !ignore_shape {
                if let Some(mut contact) = shape.get_contact(&ray) {
                    let contact_distance = ray.origin.distance_to_sqr(&contact.position);
                    if contact_distance < closest_contact_distance {
                        contact.shape_id = shape_id;
                        closest_contact = Some(contact);
                        closest_contact_distance = contact_distance;
                    }
                }
            }
        }

        closest_contact
    }
}
