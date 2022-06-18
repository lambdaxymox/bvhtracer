use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Triangle {
    pub vertex0: Vector3<f32>,
    pub vertex1: Vector3<f32>,
    pub vertex2: Vector3<f32>,
    pub centroid: Vector3<f32>,
}

impl Triangle {
    pub fn new(vertex0: Vector3<f32>, vertex1: Vector3<f32>, vertex2: Vector3<f32>, centroid: Vector3<f32>) -> Self {
        Self { vertex0, vertex1, vertex2, centroid, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, t: f32) -> Self {
        Self { origin, direction, t, }
    }

    pub fn from_origin_dir(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self::new(origin, direction, f32::MAX)
    }
}

