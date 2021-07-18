#[macro_use]
extern crate glium;

mod camera;
mod display;
mod jobs;
mod primitive;
mod raytracing;
mod shape;
mod texture;

use display::run;
use primitive::{color::Color, vector::Vector};
use raytracing::runner::RaytracingRunner;
use shape::{
    cube::CubeShape, sphere::SphereShape, ShapeProperties, Shininess,
    Transparency,
};

fn main() {
    let mut scene = RaytracingRunner::new(600, 500, Vector::new(1.5, 0.0, -1.2));
    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(1.0, 0.2, 0.2),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(0.0, 0.0, 0.0),
        1.0,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(1.0, 0.2, 0.2),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(0.0, 0.0, -3.0),
        1.0,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shininess: None,
            transparency: Some(Transparency {
                value: 1.0,
                density: 1.6,
            }),
        },
        Vector::new(1.5, 0.0, -1.2),
        0.2,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(0.0, 0.3, 1.0),
            shininess: None,
            transparency: None,
        },
        Vector::new(0.0, 1.0, -2.0),
        0.2,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(1.0, 0.4, 0.8),
            shininess: None,
            transparency: None,
        },
        Vector::new(1.0, 0.0, -2.0),
        0.3,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(0.01, 0.01, 0.01),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(0.0, 0.7, -1.1),
        0.1,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(0.01, 0.01, 0.01),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(0.9, 0.3, -0.8),
        0.1,
    ));

    scene.add_shape(SphereShape::new(
        ShapeProperties {
            color: Color::new(1.0, 1.0, 1.0),
            shininess: None,
            transparency: None,
        },
        Vector::new(3.0, 1.0, 0.0),
        0.7,
    ));

    scene.add_shape(CubeShape::new_inverted(
        ShapeProperties {
            color: Color::new(0.01, 0.01, 0.01),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(1.0, 1.0, -1.5),
        6.0,
        6.0,
        4.0,
    ));

    scene.add_shape(CubeShape::new(
        ShapeProperties {
            color: Color::new(0.01, 0.01, 0.01),
            shininess: Some(Shininess {
                value: 1.0,
                roughness: 1.0,
            }),
            transparency: None,
        },
        Vector::new(2.0, -0.75, -2.5),
        0.5,
        0.5,
        0.5,
    ));

    run(scene);
}
