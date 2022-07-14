use crate::aabb::*;
use crate::triangle::*;
use crate::ray::*;
use crate::bvh::*;


#[derive(Clone)]
pub struct Model {
    pub mesh: Vec<Triangle<f32>>,
    pub bvh: Bvh,
}

impl Model {
    pub fn new(mesh: Vec<Triangle<f32>>, bvh: Bvh) -> Self {
        Self { mesh, bvh, }
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.bvh.intersect(&self.mesh, ray)
    }

    pub fn refit(&mut self) {
        self.bvh.refit(&self.mesh)
    }

    /// Returns the model space bounds for a model.
    pub fn bounds(&self) -> Aabb<f32> {
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

    pub fn build(mut self) -> Model {
        let bvh = self.bvh_builder.build_for(&mut self.mesh);

        Model::new(self.mesh, bvh)
    }
}

