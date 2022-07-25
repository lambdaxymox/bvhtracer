use crate::geometry::*;
use cglinalg::{
    Vector2,
    Vector3,
    SimdScalar,
};
use std::io;
use std::mem;
use std::ops;
use std::slice;


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextureCoordinates<S, const N: usize> {
    data: [Vector2<S>; N],
}

impl<S, const N: usize> TextureCoordinates<S, N> {
    fn len(&self) -> usize {
        N
    }
}

impl<S, const N: usize> From<[Vector2<S>; N]> for TextureCoordinates<S, N> {
    fn from(data: [Vector2<S>; N]) -> Self {
        Self { data, }
    }
}

impl<S, const N: usize> Default for TextureCoordinates<S, N>
where
    S: SimdScalar,
{
    fn default() -> Self {
        Self { 
            data: [Vector2::zero(); N],
        }
    }
}

impl<S, const N: usize> ops::Index<usize> for TextureCoordinates<S, N> {
    type Output = Vector2<S>;

    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<S, const N: usize> ops::IndexMut<usize> for TextureCoordinates<S, N> {
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.data[_index]
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Normals<S, const N: usize> {
    data: [Vector3<S>; N],
}

impl<S, const N: usize> Normals<S, N> {
    fn len(&self) -> usize {
        N
    }
}

impl<S, const N: usize> From<[Vector3<S>; N]> for Normals<S, N> {
    fn from(data: [Vector3<S>; N]) -> Self {
        Self { data, }
    }
}
    
impl<S, const N: usize> Default for Normals<S, N>
where
    S: SimdScalar,
{
    fn default() -> Self {
        Self { 
            data: [Vector3::zero(); N],
        }
    }
}

impl<S, const N: usize> ops::Index<usize> for Normals<S, N> {
    type Output = Vector3<S>;

    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<S, const N: usize> ops::IndexMut<usize> for Normals<S, N> {
    fn index_mut(&mut self, _index: usize) -> &mut Self::Output {
        &mut self.data[_index]
    }
}


// TODO: Add tangent and bitangent vectors.
#[derive(Clone, Debug)]
pub struct Mesh<S> 
where
    S: SimdScalar,
{
    vertices: Vec<Vector3<S>>,
    tex_coords: Vec<Vector2<S>>,
    normals: Vec<Vector3<S>>,
}

impl<S> Mesh<S> 
where
    S: SimdScalar,
{
    pub(crate) fn from_parts(vertices: Vec<Vector3<S>>, tex_coords: Vec<Vector2<S>>, normals: Vec<Vector3<S>>) -> Self {
        Self { vertices, tex_coords, normals, }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn len_primitives(&self) -> usize {
        self.vertices.len() / 3
    }

    pub fn primitives(&self) -> &[Triangle<S>] {
        unsafe {
            let p = self.vertices.as_ptr() as *const Triangle<S>;
            let len = self.vertices.len() / 3;

            slice::from_raw_parts(p, len)
        }
    }

    pub fn primitives_mut(&mut self) -> &mut [Triangle<S>] {
        unsafe {
            let p = self.vertices.as_ptr() as *mut Triangle<S>;
            let len = self.vertices.len() / 3;

            slice::from_raw_parts_mut(p, len)
        }
    }

    pub fn tex_coords(&self) -> &[TextureCoordinates<S, 3>] {
        unsafe {
            let p = self.tex_coords.as_ptr() as *const TextureCoordinates<S, 3>;
            let len = self.tex_coords.len() / 2;

            slice::from_raw_parts(p, len)
        }
    }

    pub fn normals(&self) -> &[Normals<S, 3>] {
        unsafe {
            let p = self.normals.as_ptr() as *const Normals<S, 3>;
            let len = self.normals.len() / 3;

            slice::from_raw_parts(p, len)
        }
    }
}


pub struct MeshBuilder<S> 
where
    S: SimdScalar,
{
    vertices: Vec<Vector3<S>>,
    tex_coords: Vec<Vector2<S>>,
    normals: Vec<Vector3<S>>,
}

impl<S> MeshBuilder<S>
where
    S: SimdScalar,
{
    pub fn new() -> Self {
        Self { 
            vertices: vec![], 
            tex_coords: vec![], 
            normals: vec![] 
        }
    }

    pub fn with_primitive(mut self, primitive: Triangle<S>, tex_coords: TextureCoordinates<S, 3>, normals: Normals<S, 3>) -> Self {
        self.vertices.push(primitive.vertex0);
        self.vertices.push(primitive.vertex1);
        self.vertices.push(primitive.vertex2);
        self.tex_coords.push(tex_coords[0]);
        self.tex_coords.push(tex_coords[1]);
        self.tex_coords.push(tex_coords[2]);
        self.normals.push(normals[0]);
        self.normals.push(normals[1]);
        self.normals.push(normals[2]);

        self
    }

    pub fn build(mut self) -> Mesh<S> {
        Mesh::from_parts(self.vertices, self.tex_coords, self.normals)
    }
}

