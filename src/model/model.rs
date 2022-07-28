use crate::intersection::*;
use crate::geometry::*;
use crate::ray::*;
use crate::model::bvh::*;
use crate::materials::*;
use crate::mesh::*;
use std::rc::{
    Rc,
};
use std::cell::{
    RefCell,
};


#[derive(Clone, Debug)]
pub struct ModelInstance {
    handle: Rc<RefCell<Model>>,
}

impl ModelInstance {
    pub fn new(mesh: Mesh<f32>, bvh: Bvh, texture: TextureImage2D) -> Self {
        Self { 
            handle: Rc::new(RefCell::new(Model::new(mesh, bvh, texture))),
        }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        self.handle.borrow().intersect(ray)
    }

    pub fn refit(&mut self) {
        self.handle.borrow_mut().refit()
    }

    /// Returns the model space bounds for a model.
    pub fn bounds(&self) -> Aabb<f32> {
        self.handle.borrow().bounds()
    }

    pub fn model(&self) -> Rc<RefCell<Model>> {
        self.handle.clone()
    }

    pub fn len(&self) -> usize {
        self.handle.borrow().len()
    }

    pub fn len_primitives(&self) -> usize {
        self.handle.borrow().len_primitives()
    }
}


#[derive(Clone, Debug)]
pub struct Model {
    mesh: Mesh<f32>,
    bvh: Bvh,
    texture: TextureImage2D,
}

impl Model {
    pub fn new(mesh: Mesh<f32>, bvh: Bvh, texture: TextureImage2D) -> Self {
        Self { mesh, bvh, texture, }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        self.bvh.intersect(&self.mesh.primitives(), ray)
    }

    pub fn refit(&mut self) {
        self.bvh.refit(&self.mesh.primitives())
    }

    /// Returns the model space bounds for a model.
    pub fn bounds(&self) -> Aabb<f32> {
        self.bvh.bounds()
    }

    pub fn mesh(&self) -> &Mesh<f32> {
        &self.mesh
    }

    pub fn primitives(&self) -> &[Triangle<f32>] {
        &self.mesh.primitives()
    }

    pub fn primitives_mut(&mut self) -> &mut [Triangle<f32>] {
        self.mesh.primitives_mut()
    }

    pub fn tex_coords(&self) -> &[TextureCoordinates<f32, 3>] {
        &self.mesh.tex_coords()
    }

    pub fn normals(&self) -> &[Normals<f32, 3>] {
        &self.mesh.normals()
    }

    pub fn len(&self) -> usize {
        self.mesh.len()
    }

    pub fn len_primitives(&self) -> usize {
        self.mesh.len_primitives()
    }
}


pub struct ModelBuilder {
    mesh: Mesh<f32>,
    bvh_builder: BvhBuilder,
    texture: TextureImage2D,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            mesh: Mesh::from_parts(vec![], vec![], vec![]),
            bvh_builder: BvhBuilder::new(),
            texture: TextureImage2D::default(),
        }
    }

    pub fn with_mesh(mut self, mesh: Mesh<f32>) -> Self {
        self.mesh = mesh;

        self
    }

    pub fn with_texture(mut self, texture: TextureImage2D) -> Self {
        self.texture = texture;

        self
    }

    pub fn build(mut self) -> ModelInstance {
        let bvh = self.bvh_builder.build_for(self.mesh.primitives_mut());

        ModelInstance::new(self.mesh, bvh, self.texture)
    }
}

