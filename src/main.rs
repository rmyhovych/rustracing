#[macro_use]
extern crate glium;

mod camera;
mod display;
mod object;
mod primitive;
mod raytracing;
mod texture;

use display::run;
use object::{cube::CubeShape, sphere::SphereShape, ShapeProperties, ShapeType};
use primitive::{color::Color, vector::Vector};
use raytracing::runner::RaytracingRunner;

fn main() {
    let mut scene = RaytracingRunner::new(600, 500, Vector::new(1.5, -0.8, -1.2));
    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 0.2, 0.2),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.3,
            },
        },
        SphereShape::new(Vector::new(0.0, 0.0, 0.0), 1.0),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 0.2, 0.2),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.3,
            },
        },
        SphereShape::new(Vector::new(0.0, 0.0, -3.0), 1.0),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(0.2, 1.0, 0.2),
            shape_type: ShapeType::Reflector {
                transparency: 1.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        SphereShape::new(Vector::new(1.5, -0.8, -1.2), 0.2),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Emitter,
        },
        SphereShape::new(Vector::new(1.2, -0.4, -1.2), 0.2),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(0.0, 0.3, 1.0),
            shape_type: ShapeType::Emitter,
        },
        SphereShape::new(Vector::new(0.0, 1.0, -2.0), 0.2),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 0.4, 0.8),
            shape_type: ShapeType::Emitter,
        },
        SphereShape::new(Vector::new(1.0, 0.0, -2.0), 0.3),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Emitter,
        },
        SphereShape::new(Vector::new(3.0, 1.0, 0.0), 0.7),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 1.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        CubeShape::new(Vector::new(1.5, -0.8, -1.6), 0.4, 0.4, 0.4),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        CubeShape::new_inverted(Vector::new(1.0, 1.0, -1.5), 6.0, 6.0, 4.0),
    );

    run(scene);
}
