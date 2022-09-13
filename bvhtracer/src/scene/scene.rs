use crate::camera::*;
use crate::query::*;
use crate::physics::*;
use super::scene_object::*;
use super::tlas::*;


pub struct Scene {
    tlas: Tlas,
    objects: Vec<SceneObject>,
    active_camera: Camera<f32, PerspectiveProjection<f32>>,
    physics: World<f32>,
}

impl Scene {
    pub fn get_unchecked(&self, index: usize) -> &SceneObject {
        &self.objects[index]
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut SceneObject {
        &mut self.objects[index]
    }

    pub fn active_camera(&self) -> &Camera<f32, PerspectiveProjection<f32>> {
        &self.active_camera
    }

    pub fn active_camera_mut(&mut self) -> &mut Camera<f32, PerspectiveProjection<f32>> {
        &mut self.active_camera
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        self.tlas.intersect(&self.objects, ray)
    }

    pub fn rebuild(&mut self) {
        self.tlas.rebuild(&self.objects);
    }

    fn run_physics(&mut self, elapsed: f64) {
        self.physics.run_physics(elapsed as f32);
        for object in self.objects.iter_mut() {
            object.update_transform();
        }
    }

    pub fn run(&mut self, elapsed: f64) {
        self.run_physics(elapsed);
        self.rebuild();
    }
}

pub struct SceneBuilder {
    objects: Vec<SceneObject>,
    physics: World<f32>,
    active_camera: Camera<f32, PerspectiveProjection<f32>>,
}

impl SceneBuilder {
    pub fn new(camera: Camera<f32, PerspectiveProjection<f32>>) -> Self {
        Self {
            objects: vec![],
            physics: World::new(),
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

    pub fn with_physics(mut self, physics: World<f32>) -> Self {
        self.physics = physics;

        self
    }

    pub fn build(self) -> Scene {
        let tlas = TlasBuilder::new()
            .build_for(&self.objects);

        Scene {
            tlas,
            objects: self.objects,
            physics: self.physics,
            active_camera: self.active_camera,
        }
    }
}

