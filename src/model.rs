use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use crate::bvh::*;
use std::rc::{
    Rc,
};
use std::cell::{
    Ref,
    RefMut,
    RefCell,
};


#[derive(Clone, Debug)]
pub struct ModelInstance {
    handle: Rc<RefCell<Model>>,
}

impl ModelInstance {
    pub fn new(mesh: Vec<Triangle<f32>>, bvh: Bvh) -> Self {
        Self { 
            handle: Rc::new(RefCell::new(Model::new(mesh, bvh))),
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

    pub fn mesh(&self) -> Rc<RefCell<Vec<Triangle<f32>>>> {
        self.handle.borrow().mesh.clone()
    }
}


#[derive(Clone, Debug)]
struct Model {
    mesh: Rc<RefCell<Vec<Triangle<f32>>>>,
    bvh: Bvh,
}

impl Model {
    fn new(mesh: Vec<Triangle<f32>>, bvh: Bvh) -> Self {
        Self { 
            mesh: Rc::new(RefCell::new(mesh)),
            bvh, 
        }
    }

    fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        let mesh = self.mesh.borrow();
        self.bvh.intersect(mesh.as_slice(), ray)
    }

    fn refit(&mut self) {
        let mesh = self.mesh.borrow();
        self.bvh.refit(mesh.as_slice())
    }

    /// Returns the model space bounds for a model.
    fn bounds(&self) -> Aabb<f32> {
        self.bvh.bounds()
    }
}

pub struct ModelBuilder {
    mesh: Vec<Triangle<f32>>,
    bvh_builder: BvhBuilder
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self {
            mesh: Vec::new(),
            bvh_builder: BvhBuilder::new(),
        }
    }

    pub fn with_mesh(mut self, mesh: Vec<Triangle<f32>>) -> Self {
        self.mesh = mesh;

        self
    }

    pub fn build(mut self) -> ModelInstance {
        let bvh = self.bvh_builder.build_for(&mut self.mesh);

        ModelInstance::new(self.mesh, bvh)
    }
}

