use crate::triangle::*;
use cglinalg::{
    Vector2,
    Vector3,
};
use std::io;

/*
#[derive(Clone, Debug)]
pub struct MeshInstance {
    mesh: Rc<RefCell<Vec<Triangle<f32>>>>,
}
*/

#[derive(Clone, Debug)]
pub struct Mesh {
    primitives: Vec<Triangle<f32>>,
    tex_coords: Vec<Vector2<f32>>,
    normals: Vec<Vector3<f32>>,
}

impl Mesh {
    pub fn new(primitives: Vec<Triangle<f32>>, tex_coords: Vec<Vector2<f32>>, normals: Vec<Vector3<f32>>) -> Self {
        Self { primitives, tex_coords, normals }
    }

    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    pub fn primitives(&self) -> &[Triangle<f32>] {
        &self.primitives
    }

    pub fn primitives_mut(&mut self) -> &mut [Triangle<f32>] {
        &mut self.primitives
    }

    pub fn tex_coords(&self) -> &[Vector2<f32>] {
        &self.tex_coords
    }

    pub fn normals(&self) -> &[Vector3<f32>] {
        &self.normals
    }
}


