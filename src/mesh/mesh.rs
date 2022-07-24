use crate::geometry::*;
use cglinalg::{
    Vector2,
    Vector3,
    SimdScalar,
};
use std::io;
use std::ops;

/*
#[derive(Clone, Debug)]
pub struct MeshInstance {
    mesh: Rc<RefCell<Vec<Triangle<f32>>>>,
}
*/

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct TextureCoordinates<S, const N: usize> {
    data: [Vector2<S>; N],
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
    primitives: Vec<Triangle<S>>,
    tex_coords: Vec<TextureCoordinates<S, 3>>,
    normals: Vec<Normals<S, 3>>,
}

impl<S> Mesh<S> 
where
    S: SimdScalar,
{
    pub fn new(primitives: Vec<Triangle<S>>, tex_coords: Vec<TextureCoordinates<S, 3>>, normals: Vec<Normals<S, 3>>) -> Self {
        Self { primitives, tex_coords, normals }
    }

    pub fn len(&self) -> usize {
        self.primitives.len()
    }

    pub fn primitives(&self) -> &[Triangle<S>] {
        &self.primitives
    }

    pub fn primitives_mut(&mut self) -> &mut [Triangle<S>] {
        &mut self.primitives
    }

    pub fn tex_coords(&self) -> &[TextureCoordinates<S, 3>] { // &[Vector2<f32>] {
        &self.tex_coords
    }

    pub fn normals(&self) -> &[Normals<S, 3>] { // &[Vector3<f32>] {
        &self.normals
    }
}

