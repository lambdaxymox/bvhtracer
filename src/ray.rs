use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub recip_direction: Vector3<f32>,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, t: f32) -> Self {
        let recip_direction = Vector3::new(
            1_f32 / direction.x, 
            1_f32 / direction.y, 
            1_f32 / direction.z
        );

        Self { origin, direction, recip_direction, t, }
    }

    pub fn from_origin_dir(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self::new(origin, direction, f32::MAX)
    }
}

