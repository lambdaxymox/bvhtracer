use crate::intersection::*;
use crate::geometry::*;
use crate::ray::*;
use crate::tlas::*;
use crate::model::{
    ModelInstance,
};
use cglinalg::{
    Magnitude,
    Matrix4x4,
    Vector3,
};


#[derive(Clone, Debug)]
pub struct SceneObject {
    model: ModelInstance,
    transform: Matrix4x4<f32>,
    transform_inv: Matrix4x4<f32>,
    bounds: Aabb<f32>,
}

impl SceneObject {
    pub fn set_transform(&mut self, transform: &Matrix4x4<f32>) {
        let transform_inv = transform.inverse().unwrap();
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            let position = Vector3::new(bounds_x, bounds_y, bounds_z).extend(1_f32);
            let new_position = (transform * position).contract();

            new_bounds.grow(&new_position);
        }

        self.transform = *transform;
        self.transform_inv = transform_inv;
        self.bounds = new_bounds;
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        // Convert the ray from world space to model space.
        let ray_model_space_origin = {
            let origin_world_space = ray.origin.extend(1_f32);
            let origin_model_space = self.transform_inv * origin_world_space;
            origin_model_space.contract()
        };
        let ray_model_space_direction = {
            let direction_world_space = ray.direction.extend(0_f32);
            let direction_model_space = self.transform_inv * direction_world_space;
            direction_model_space.contract()
        }; 
        let ray_model_space = Ray::new(ray_model_space_origin, ray_model_space_direction, ray.t);
    
        self.model.intersect(&ray_model_space)
    }

    /// Returns the world space bounds for a scene object.
    #[inline]
    pub fn bounds(&self) -> Aabb<f32> {
        self.bounds
    }

    #[inline]
    pub fn model(&self) -> ModelInstance {
        self.model.clone()
    }
}

pub struct SceneObjectBuilder {
    model: ModelInstance,
    transform: Matrix4x4<f32>,
    transform_inv: Matrix4x4<f32>,
    bounds: Aabb<f32>,
}

impl SceneObjectBuilder {
    pub fn new(model: ModelInstance) -> Self {
        Self { 
            model,
            transform: Matrix4x4::identity(), 
            transform_inv: Matrix4x4::identity(), 
            bounds: Aabb::new_empty() 
        }
    }

    pub fn with_transform(mut self, transform: &Matrix4x4<f32>) -> Self {
        let transform_inv = transform.inverse().unwrap();
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            let position = Vector3::new(bounds_x, bounds_y, bounds_z).extend(1_f32);
            let new_position = (transform * position).contract();

            new_bounds.grow(&new_position);
        }

        self.transform = *transform;
        self.transform_inv = transform_inv;
        self.bounds = new_bounds;

        self
    }

    pub fn build(self) -> SceneObject {
        SceneObject { 
            model: self.model, 
            transform: self.transform, 
            transform_inv: self.transform_inv, 
            bounds: self.bounds 
        }
    }
}

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

pub struct Scene {
    tlas: Tlas,
    active_camera: Camera,
}

impl Scene {
    pub fn get_unchecked(&self, index: usize) -> &SceneObject {
        self.tlas.get_unchecked(index)
    }

    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut SceneObject {
        self.tlas.get_mut_unchecked(index)
    }

    pub fn active_camera(&self) -> &Camera {
        &self.active_camera
    }

    pub fn active_camera_mut(&mut self) -> &mut Camera {
        &mut self.active_camera
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.tlas.intersect(ray)
    }

    pub fn rebuild(&mut self) {
        self.tlas.rebuild();
    }
}

pub struct SceneBuilder {
    objects: Vec<SceneObject>,
    active_camera: Camera,
}

impl SceneBuilder {
    pub fn new(camera: Camera) -> Self {
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

