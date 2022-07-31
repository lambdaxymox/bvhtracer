use crate::camera::*;
use crate::intersection::*;
use crate::ray::*;
use crate::scene_object::*;
use crate::tlas::*;

use cglinalg::{
    Magnitude,
    Vector3,
};

/*
pub struct IdentityModelSpec {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    plane: f32,
    position: f32,
    /// The offset of the focal point of the camera from the camera's origin.
    focal_offset: Vector3<f32>,
}

impl IdentityModelSpec {
    pub fn new(
        left: f32, 
        right: f32, 
        bottom: f32, 
        top: f32,
        plane: f32,
        position: f32, 
        focal_offset: Vector3<f32>) -> Self 
    {
        Self { left, right, bottom, top, plane, position, focal_offset, }
    }
}

pub struct Camera {
    position: Vector3<f32>,
    /// The focal point of the camera, i.e. the location from which we cast rays into the scene 
    /// from the camera.
    focal_point: Vector3<f32>,
    top_left: Vector3<f32>,
    top_right: Vector3<f32>,
    bottom_left: Vector3<f32>,
}

impl Camera {
    pub fn from_spec(spec: IdentityModelSpec) -> Self {
        let position_z = spec.position;
        let position_x = (spec.left + spec.right) / 2_f32; 
        let position_y = (spec.bottom + spec.top) / 2_f32;
        let position = Vector3::new(position_x, position_y, position_z);
        let focal_point = position + spec.focal_offset;
        let top_left = Vector3::new(spec.left, spec.top, spec.plane);
        let top_right = Vector3::new(spec.right, spec.top, spec.plane);
        let bottom_left = Vector3::new(spec.left, spec.bottom, spec.plane);
        

        Self { position, focal_point, top_left, top_right, bottom_left, }
    }

    pub fn get_ray_world_space(&self, u: f32, v: f32) -> Ray<f32> {
        let ray_origin = self.focal_point;
        let pixel_position = ray_origin + 
            self.top_left + 
            (self.top_right - self.top_left) * u + 
            (self.bottom_left - self.top_left) * v;
        let ray_direction = (pixel_position - ray_origin).normalize();

        Ray::from_origin_dir(ray_origin, ray_direction)
    }
}
*/
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

