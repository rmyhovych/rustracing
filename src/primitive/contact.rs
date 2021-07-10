use super::vector::Vector;

pub struct RayContact {
    pub shape_id: usize,
    pub position: Vector,
    pub normal: Vector,
}

impl RayContact {
    pub fn new(position: Vector, normal: Vector) -> Self {
        Self {
            shape_id: 0,
            position,
            normal,
        }
    }
}
