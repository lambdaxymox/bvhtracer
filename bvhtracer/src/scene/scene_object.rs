use crate::transform::*;
use crate::transform_component::*;
use crate::geometry::*;
use crate::query::*;
use crate::model::{
    ModelInstance,
};
use crate::physics::{
    RigidBodyInstance,
};
use cglinalg::{
    Matrix4x4,
    Vector3,
};


#[derive(Clone, Debug)]
pub struct SceneObject {
    model: ModelInstance,
    rigid_body: RigidBodyInstance<f32>,
    transform_init: Transform3<f32>,
    transform_component: TransformComponent3<f32>,
    bounds: Aabb<f32>,
}

impl SceneObject {
    #[inline]
    pub const fn get_transform(&self) -> &Transform3<f32> {
        &self.transform_component.transform()
    }

    #[inline]
    pub const fn get_transform_inv(&self) -> &Transform3<f32> {
        self.transform_component.transform_inv()
    }

    /// Returns the world space bounds for a scene object.
    #[inline]
    pub const fn bounds(&self) -> Aabb<f32> {
        self.bounds
    }

    #[inline]
    pub fn model(&self) -> ModelInstance {
        self.model.clone()
    }

    #[inline]
    pub (crate) fn physics(&self) -> RigidBodyInstance<f32> {
        self.rigid_body.clone()
    }

    pub fn update_transform(&mut self) {
        let rigid_body = self.rigid_body.rc();
        let borrow = rigid_body.borrow();
        let rigid_body_transform = borrow.get_transform();
        // let scale = self.transform_component.transform().scale;
        /*
        let new_transform = Transform3::new(
            scale,
            rigid_body_transform.translation,
            rigid_body_transform.rotation
        );
        self.transform_component.set_transform(&new_transform);
        */
        let new_transform = Transform3::new(
            rigid_body_transform.scale * self.transform_init.scale,
            &(rigid_body_transform.translation + self.transform_init.translation),
            rigid_body_transform.rotation * self.transform_init.rotation,
        );
        self.transform_component.set_transform(&new_transform);
    }

    pub fn set_transform(&mut self, transform: &Transform3<f32>) {
    // pub fn set_transform(&mut self, transform: &Matrix4x4<f32>) {
        // let transform_inv = transform.inverse().unwrap();
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            /*
            let position = Vector3::new(bounds_x, bounds_y, bounds_z).extend(1_f32);
            let new_position = (transform * position).contract();
            */
            let position = Vector3::new(bounds_x, bounds_y, bounds_z);
            let new_position = transform.transform_point(&position);

            new_bounds.grow(&new_position);
        }

        self.transform_component.set_transform(transform);
        /*
        self.transform = *transform;
        self.transform_inv = transform_inv;
        */
        self.bounds = new_bounds;
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        // Convert the ray from world space to model space.
        let ray_model_space_origin = {
            /*
            let origin_world_space = ray.origin.extend(1_f32);
            let origin_model_space = self.transform_inv * origin_world_space;
            origin_model_space.contract()
            */
            self.get_transform_inv().transform_point(&ray.origin)
        };
        let ray_model_space_direction = {
            /*
            let direction_world_space = ray.direction.extend(0_f32);
            let direction_model_space = self.transform_inv * direction_world_space;
            direction_model_space.contract()
            */
            self.get_transform_inv().transform_vector(&ray.direction)
        }; 
        let ray_model_space = Ray::new(ray_model_space_origin, ray_model_space_direction, ray.t);
    
        self.model.intersect(&ray_model_space)
    }
}

pub struct SceneObjectBuilder {
    model: ModelInstance,
    rigid_body: RigidBodyInstance<f32>,
    /*
    transform: Matrix4x4<f32>,
    transform_inv: Matrix4x4<f32>,
    */
    transform: Transform3<f32>,
    bounds: Aabb<f32>,
}

impl SceneObjectBuilder {
    pub fn new(model: ModelInstance, rigid_body: RigidBodyInstance<f32>) -> Self {
        Self { 
            model,
            rigid_body,
            /*
            transform: Matrix4x4::identity(), 
            transform_inv: Matrix4x4::identity(), 
            */
            transform: Transform3::identity(),
            bounds: Aabb::new_empty() 
        }
    }

    pub fn with_transform(mut self, transform: &Transform3<f32>) -> Self {
    // pub fn with_transform(mut self, transform: &Matrix4x4<f32>) -> Self {
        let transform_inv = transform.inverse().unwrap();
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            /*
            let position = Vector3::new(bounds_x, bounds_y, bounds_z).extend(1_f32);
            let new_position = (transform * position).contract();
            */
            let position = Vector3::new(bounds_x, bounds_y, bounds_z);
            let new_position = transform.transform_point(&position);

            new_bounds.grow(&new_position);
        }

        self.transform = *transform;
        // self.transform_inv = transform_inv;
        self.bounds = new_bounds;

        self
    }

    pub fn build(self) -> SceneObject {
        SceneObject { 
            model: self.model, 
            rigid_body: self.rigid_body,
            transform_init: self.transform,
            transform_component: TransformComponent3::new(self.transform),
            bounds: self.bounds 
        }
    }
}

