use std::f32::consts::PI;

use rand::{thread_rng, Rng};

use crate::{
    primitive::{color::Color, contact::RayContact, ray::Ray, vector::Vector},
    shape::Shape,
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
        self.project_ray(&ray, 0)
    }

    pub fn project_ray(&self, ray: &Ray, recursive_index: u32) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        if recursive_index < self.max_bounce_count {
            if let Some(contact) = self.find_closest_contact(&ray) {
                let shape = &self.shapes[contact.get_shape()];
                let shape_properties = shape.get_shape_properties();

                let from_inside = contact.is_from_inside();

                let mut shape_color_retention = 1.0;
                if !from_inside {
                    if let Some(shininess) = &shape_properties.shininess {
                        let new_ray = contact.get_random_outer_reflection();

                        let reflected_color = self.project_ray(&new_ray, recursive_index + 1);
                        color.r += shape_color_retention * shininess.value * reflected_color.r;
                        color.g += shape_color_retention * shininess.value * reflected_color.g;
                        color.b += shape_color_retention * shininess.value * reflected_color.b;

                        shape_color_retention *= 1.0 - shininess.value;
                    }
                }

                if let Some(transparency) = &shape_properties.transparency {
                    let mut index_incident = 1.0;
                    let mut index_refracted = 1.0;
                    if from_inside {
                        index_incident = transparency.density;
                    } else {
                        index_refracted = transparency.density;
                    }

                    let new_ray = contact.get_refraction(index_incident, index_refracted);
                    let reflected_color = self.project_ray(&new_ray, recursive_index + 1);

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

    fn find_closest_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        let mut closest_contact: Option<RayContact<'a>> = None;
        let mut closest_contact_distance = f32::MAX;
        for (shape_id, shape) in self.shapes.iter().enumerate() {
            if let Some(mut contact) = shape.get_contact(&ray) {
                let contact_distance = contact.get_distance_from_origin();
                if contact_distance < closest_contact_distance {
                    contact.set_shape(shape_id);
                    closest_contact = Some(contact);
                    closest_contact_distance = contact_distance;
                }
            }
        }

        closest_contact
    }
}
