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

#[derive(Copy, Clone, Debug)]
pub struct TriangleTexCoords {
    pub uv0: Vector2<f32>,
    pub uv1: Vector2<f32>,
    pub uv2: Vector2<f32>,
}

impl TriangleTexCoords {
    pub fn new(uv0: Vector2<f32>, uv1: Vector2<f32>, uv2: Vector2<f32>) -> Self {
        Self { uv0, uv1, uv2, }
    }
}

impl Default for TriangleTexCoords {
    fn default() -> Self {
        Self { 
            uv0: Vector2::zero(), 
            uv1: Vector2::zero(), 
            uv2: Vector2::zero() 
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TriangleNormals {
    pub normal0: Vector3<f32>,
    pub normal1: Vector3<f32>,
    pub normal2: Vector3<f32>,
}

impl TriangleNormals {
    pub fn new(normal0: Vector3<f32>, normal1: Vector3<f32>, normal2: Vector3<f32>) -> Self {
        Self { normal0, normal1, normal2, }
    }
}

impl Default for TriangleNormals {
    fn default() -> Self {
        Self { 
            normal0: Vector3::zero(), 
            normal1: Vector3::zero(), 
            normal2: Vector3::zero() 
        }
    }
}

// TODO: Add tangent and bitangent vectors.
#[derive(Clone, Debug)]
pub struct Mesh {
    primitives: Vec<Triangle<f32>>,
    tex_coords: Vec<TriangleTexCoords>,
    normals: Vec<TriangleNormals>,
}

impl Mesh {
    pub fn new(primitives: Vec<Triangle<f32>>, tex_coords: Vec<TriangleTexCoords>, normals: Vec<TriangleNormals>) -> Self {
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

    pub fn tex_coords(&self) -> &[TriangleTexCoords] { // &[Vector2<f32>] {
        &self.tex_coords
    }

    pub fn normals(&self) -> &[TriangleNormals] { // &[Vector3<f32>] {
        &self.normals
    }
}

