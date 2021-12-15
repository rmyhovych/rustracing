use std::f32::consts::PI;

use crate::{
    object::{Object, ShapeType},
    primitive::{color::Color, contact::RayContact, ray::Ray},
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
    objects: Vec<Box<dyn Object>>,

    max_bounce_count: usize,
}

impl RaytracingScene {
    pub fn new(max_bounce_count: usize) -> Self {
        Self {
            objects: Vec::new(),
            max_bounce_count,
        }
    }

    pub fn add_object<O: Object + 'static>(&mut self, object: O) {
        self.objects.push(Box::new(object));
    }

    pub fn get_pixel_color(&self, ray: Ray) -> Color {
        self.project_ray(ray, self.max_bounce_count)
    }

    fn project_ray(&self, ray: Ray, bounces_left: usize) -> Color {
        if bounces_left == 0 {
            Color::zero()
        } else if let Some(contact) = self.find_closest_contact(&ray) {
            let object = &self.objects[contact.get_object_id()];
            let properties = object.get_properties();

            match properties.shape_type {
                ShapeType::Emitter => properties.color.clone(),
                ShapeType::Reflector {
                    transparency,
                    roughness,
                    density,
                } => {
                    let from_inside = contact.is_from_inside();

                    if from_inside {
                        let inside_refraction_ray = contact.get_refraction(density, 1.0);
                        self.project_ray(inside_refraction_ray, bounces_left - 1)
                    } else {
                        let mut color = Color::zero();
                        if transparency < 1.0 {
                            let reflection_ray = contact.get_outer_reflection(roughness);
                            let reflection_color =
                                self.project_ray(reflection_ray, bounces_left - 1);
                            color = color.plus(&reflection_color.times(1.0 - transparency));
                        }

                        if transparency > 0.0 {
                            let refraction_ray = contact.get_refraction(1.0, density);
                            let refraction_color =
                                self.project_ray(refraction_ray, bounces_left - 1);
                            color = color.plus(&refraction_color.times(transparency));
                        }

                        color.filter(&properties.color)
                    }
                }
            }
        } else {
            Color::zero()
        }
    }

    fn find_closest_contact<'a>(&self, ray: &'a Ray) -> Option<RayContact<'a>> {
        let mut closest_contact: Option<RayContact<'a>> = None;
        let mut closest_contact_distance = f32::MAX;
        for (object_id, object) in self.objects.iter().enumerate() {
            if let Some(mut contact) = object.get_contact(&ray) {
                let contact_distance = contact.get_distance_from_origin();
                if contact_distance < closest_contact_distance {
                    contact.set_object_id(object_id);
                    closest_contact = Some(contact);
                    closest_contact_distance = contact_distance;
                }
            }
        }

        closest_contact
    }
}
