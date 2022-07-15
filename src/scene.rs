use crate::aabb::*;
use crate::ray::*;
use crate::tlas::*;
use crate::model::{
    Model,
};
use cglinalg::{
    Matrix4x4,
    Vector3,
};


#[derive(Clone, Debug)]
pub struct SceneObject {
    model: Model,
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
    pub fn model(&self) -> &Model {
        &self.model
    }

    #[inline]
    pub fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }
}

pub struct SceneObjectBuilder {
    model: Model,
    transform: Matrix4x4<f32>,
    transform_inv: Matrix4x4<f32>,
    bounds: Aabb<f32>,
}

impl SceneObjectBuilder {
    pub fn new(model: Model) -> Self {
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

pub struct Scene {
    tlas: Tlas,
}

impl Scene {
    #[inline]
    pub fn get_unchecked(&self, index: usize) -> &SceneObject {
        self.tlas.get_unchecked(index)
    }

    #[inline]
    pub fn get_mut_unchecked(&mut self, index: usize) -> &mut SceneObject {
        self.tlas.get_mut_unchecked(index)
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<f32>) -> Option<f32> {
        self.tlas.intersect(ray)
    }

    #[inline]
    pub fn rebuild(&mut self) {
        self.tlas.rebuild();
    }
}

pub struct SceneBuilder {
    objects: Vec<SceneObject>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self { 
            objects: vec![],
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
        }
    }
}

