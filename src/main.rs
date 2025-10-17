#[macro_use]
extern crate glium;

mod camera;
mod display;
mod object;
mod primitive;
mod raytracing;
mod texture;

use std::f32::consts::PI;

use display::run;
use object::{cube::CubeShape, plane::PlaneShape, sphere::SphereShape, ShapeProperties, ShapeType};
use primitive::{color::Color, vector::Vector};
use raytracing::runner::RaytracingRunner;

fn main() {
    let mut scene = RaytracingRunner::new(600, 500, Vector::new(0.0, 0.0, 0.0));

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        PlaneShape::new(
            Vector::new(0.0, 1.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            PI,
            2.0,
            2.0,
        ),
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
        PlaneShape::new(
            Vector::new(0.0, -1.0, 0.0),
            Vector::new(1.0, 0.0, 0.0),
            0.0,
            2.0,
            2.0,
        ),
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
        PlaneShape::new(
            Vector::new(0.0, 0.0, 1.0),
            Vector::new(1.0, 0.0, 0.0),
            -PI / 2.0,
            2.0,
            2.0,
        ),
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
        PlaneShape::new(
            Vector::new(0.0, 0.0, -1.0),
            Vector::new(1.0, 0.0, 0.0),
            PI / 2.0,
            2.0,
            2.0,
        ),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 0.3, 0.3),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        PlaneShape::new(
            Vector::new(1.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            PI / 2.0,
            2.0,
            2.0,
        ),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(0.3, 1.0, 0.3),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        PlaneShape::new(
            Vector::new(-1.0, 0.0, 0.0),
            Vector::new(0.0, 0.0, 1.0),
            -PI / 2.0,
            2.0,
            2.0,
        ),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Emitter,
        },
        CubeShape::new(Vector::new(0.0, 0.995, 0.0), 0.5, 0.5, 0.01),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 0.75,
                roughness: 1.0,
                density: 1.6,
            },
        },
        SphereShape::new(Vector::new(0.6, -0.75, -0.5), 0.3),
    );

    /*
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

    /*
    scene.add_object(
        ShapeProperties {
            color: Color::new(0.2, 0.2, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 1.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        SphereShape::new(Vector::new(1.9, -0.8, -1.2), 0.2),
    );

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 0.2, 0.2),
            shape_type: ShapeType::Reflector {
                transparency: 1.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        SphereShape::new(Vector::new(1.1, -0.8, -1.2), 0.2),
    );
    */

    scene.add_object(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 0.0,
                roughness: 1.0,
                density: 1.6,
            },
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
            color: Color::new(0.5, 1.0, 1.0),
            shape_type: ShapeType::Reflector {
                transparency: 1.0,
                roughness: 1.0,
                density: 1.6,
            },
        },
        CubeShape::new(Vector::new(1.5, -0.8, -1.8), 0.4, 0.4, 0.4),
    );
    */

    run(scene);
}
