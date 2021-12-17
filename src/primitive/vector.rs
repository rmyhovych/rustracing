use std::f32::consts::PI;

use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from(other: &Vector) -> Self {
        Self {
            x: other.x,
            y: other.y,
            z: other.z,
        }
    }

    pub fn z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn plus(&self, other: &Vector) -> Self {
        return Self::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }

    pub fn minus(&self, other: &Vector) -> Self {
        return Self::new(self.x - other.x, self.y - other.y, self.z - other.z);
    }

    pub fn multiply(&self, other: &Vector) -> Self {
        return Self::new(self.x * other.x, self.y * other.y, self.z * other.z);
    }

    pub fn dot(&self, other: &Vector) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn angle_between(&self, other: &Vector) -> f32 {
        (self.dot(&other) / (self.len() * other.len())).acos()
    }

    pub fn cross(&self, other: &Vector) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn times(&self, x: f32) -> Self {
        Self::new(x * self.x, x * self.y, x * self.z)
    }

    pub fn project_onto(&self, other: &Vector) -> Self {
        other.times(self.dot(other) / other.dot(other))
    }

    pub fn rotate_around_vector(&mut self, vector: &Vector, angle: f32) {
        if self.dot(&vector) != self.len() * vector.len() {
            let parallel = self.project_onto(vector);
            let perpendicular_x = self.minus(&parallel);
            let mut perpendicular_y = vector.cross(&perpendicular_x);
            perpendicular_y.normalize_to(perpendicular_x.len());

            let rotated = parallel
                .plus(&perpendicular_x.times(angle.cos()))
                .plus(&perpendicular_y.times(angle.sin()));

            self.x = rotated.x;
            self.y = rotated.y;
            self.z = rotated.z;
        }
    }

    pub fn random_perpendicular(&self) -> Vector {
        let arbitrary: [f32; 2] = [1.0, 1.0];

        let mut perpendicular = if self.x != 0.0 {
            let perp_x = (-self.y * arbitrary[0] - self.z * arbitrary[1]) / self.x;
            Vector::new(perp_x, arbitrary[0], arbitrary[1])
        } else if self.y != 0.0 {
            let perp_y = (-self.x * arbitrary[0] - self.z * arbitrary[1]) / self.y;
            Vector::new(arbitrary[0], perp_y, arbitrary[1])
        } else if self.z != 0.0 {
            let perp_z = (-self.x * arbitrary[0] - self.y * arbitrary[1]) / self.z;
            Vector::new(arbitrary[0], arbitrary[1], perp_z)
        } else {
            Vector::new(0.0, 0.0, 0.0)
        };

        perpendicular.rotate_around_vector(&self, thread_rng().gen_range(0.0, 2.0 * PI));
        perpendicular
    }

    pub fn normalized(&self) -> Self {
        let mut new_vec = Self::from(&self);
        new_vec.normalize();

        new_vec
    }

    pub fn normalized_to(&self, new_len: f32) -> Self {
        let mut new_vec = Self::from(&self);
        new_vec.normalize_to(new_len);

        new_vec
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalize_to(&mut self, new_len: f32) {
        let div = self.len() / new_len;
        self.x /= div;
        self.y /= div;
        self.z /= div;
    }

    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    pub fn len_sqr(&self) -> f32 {
        self.dot(&self)
    }

    pub fn distance_to(&self, other: &Vector) -> f32 {
        self.distance_to_sqr(other).sqrt()
    }

    pub fn distance_to_sqr(&self, other: &Vector) -> f32 {
        let towards = Vector::new(other.x - self.x, other.y - self.y, other.z - self.z);
        towards.len_sqr()
    }

    pub fn sum(&self) -> f32 {
        return self.x + self.y + self.z;
    }
}
