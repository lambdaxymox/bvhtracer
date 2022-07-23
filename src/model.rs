use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use crate::bvh::*;
use crate::texture::*;
use crate::mesh::*;
use cglinalg::{
    Vector2,
    Vector3,
};
use std::rc::{
    Rc,
};
use std::cell::{
    Ref,
    RefMut,
    RefCell,
};
use std::slice;


#[derive(Clone, Debug)]
pub struct ModelInstance {
    handle: Rc<RefCell<Model>>,
}

impl ModelInstance {
    pub fn new(mesh: Mesh, bvh: Bvh, texture: TextureImage2D) -> Self {
        Self { 
            handle: Rc::new(RefCell::new(Model::new(mesh, bvh, texture))),
        }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.handle.borrow().intersect(ray)
    }

    pub fn refit(&mut self) {
        self.handle.borrow_mut().refit()
    }

    /// Returns the model space bounds for a model.
    pub fn bounds(&self) -> Aabb<f32> {
        self.handle.borrow().bounds()
    }

    pub fn mesh(&self) -> Rc<RefCell<Model>> {
        self.handle.clone()
    }

    pub fn len(&self) -> usize {
        self.handle.borrow().len()
    }
}


#[derive(Clone, Debug)]
pub struct Model {
    mesh: Mesh,
    bvh: Bvh,
    texture: TextureImage2D,
}

impl Model {
    pub fn new(mesh: Mesh, bvh: Bvh, texture: TextureImage2D) -> Self {
        Self { mesh, bvh, texture, }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.bvh.intersect(&self.mesh.primitives(), ray)
    }

    pub fn refit(&mut self) {
        self.bvh.refit(&self.mesh.primitives())
    }

    /// Returns the model space bounds for a model.
    pub fn bounds(&self) -> Aabb<f32> {
        self.bvh.bounds()
    }

    pub fn primitives(&self) -> &[Triangle<f32>] {
        &self.mesh.primitives()
    }

    pub fn primitives_mut(&mut self) -> &mut [Triangle<f32>] {
        self.mesh.primitives_mut()
    }

    pub fn tex_coords(&self) -> &[Vector2<f32>] {
        &self.mesh.tex_coords()
    }

    pub fn normals(&self) -> &[Vector3<f32>] {
        &self.mesh.normals()
    }

    pub fn len(&self) -> usize {
        self.mesh.len()
    }
}

pub struct ModelBuilder {
    primitives: Vec<Triangle<f32>>,
    tex_coords: Vec<Vector2<f32>>,
    normals: Vec<Vector3<f32>>,
    bvh_builder: BvhBuilder,
    texture: TextureImage2D,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            tex_coords: Vec::new(),
            normals: Vec::new(),
            bvh_builder: BvhBuilder::new(),
            texture: TextureImage2D::default(),
        }
    }

    pub fn with_primitives(mut self, primitives: Vec<Triangle<f32>>) -> Self {
        self.primitives = primitives;

        self
    }

    pub fn with_tex_coords(mut self, tex_coords: Vec<Vector2<f32>>) -> Self {
        self.tex_coords = tex_coords;

        self
    }

    pub fn with_normals(mut self, normals: Vec<Vector3<f32>>) -> Self {
        self.normals = normals;

        self
    }

    pub fn with_texture(mut self, texture: TextureImage2D) -> Self {
        self.texture = texture;

        self
    }

    pub fn build(mut self) -> ModelInstance {
        let bvh = self.bvh_builder.build_for(&mut self.primitives);
        let mesh = Mesh::new(self.primitives, self.tex_coords, self.normals);

        ModelInstance::new(mesh, bvh, self.texture)
    }
}

