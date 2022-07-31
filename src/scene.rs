use crate::camera::*;
use crate::intersection::*;
use crate::ray::*;
use crate::scene_object::*;
use crate::tlas::*;

use cglinalg::{
    Magnitude,
    Vector3,
};


pub struct Scene {
    tlas: Tlas,
    active_camera: Camera<f32, PerspectiveProjection<f32>>,
}

impl Scene {
    pub fn get_unchecked(&self, index: usize) -> &SceneObject {
        self.tlas.get_unchecked(index)
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut SceneObject {
        self.tlas.get_mut_unchecked(index)
    }

    pub fn active_camera(&self) -> &Camera<f32, PerspectiveProjection<f32>> {
        &self.active_camera
    }

    pub fn active_camera_mut(&mut self) -> &mut Camera<f32, PerspectiveProjection<f32>> {
        &mut self.active_camera
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        self.tlas.intersect(ray)
    }

    pub fn rebuild(&mut self) {
        self.tlas.rebuild();
    }
}

pub struct SceneBuilder {
    objects: Vec<SceneObject>,
    active_camera: Camera<f32, PerspectiveProjection<f32>>,
}

impl SceneBuilder {
    pub fn new(camera: Camera<f32, PerspectiveProjection<f32>>) -> Self {
        Self {
            objects: vec![],
            active_camera: camera,
        }
    }

    pub fn with_object(mut self, object: SceneObject) -> Self {
        self.objects.push(object);

        self
    }

    pub fn with_objects(mut self, new_objects: Vec<SceneObject>) -> Self {
        self.objects = new_objects;

        self
    }

    pub fn build(self) -> Scene {
        let tlas = TlasBuilder::new()
            .with_objects(self.objects)
            .build();

        Scene {
            tlas,
            active_camera: self.active_camera,
        }
    }
}

