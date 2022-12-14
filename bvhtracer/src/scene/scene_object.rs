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
        let new_transform = rigid_body_transform * self.transform_init;
        self.set_transform(&new_transform);
    }

    pub fn set_transform(&mut self, transform: &Transform3<f32>) {
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            let position = Vector3::new(bounds_x, bounds_y, bounds_z);
            let new_position = transform.transform_point(&position);

            new_bounds.grow(&new_position);
        }

        self.transform_component.set_transform(transform);
        self.bounds = new_bounds;
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<f32>) -> Option<Intersection<f32>> {
        // Convert the ray from world space to model space.
        let ray_model_space_origin = self
            .get_transform_inv()
            .transform_point(&ray.origin);
        let ray_model_space_direction = self
            .get_transform_inv()
            .transform_vector(&ray.direction); 
        let ray_model_space = Ray::new(ray_model_space_origin, ray_model_space_direction, ray.t);
    
        self.model.intersect(&ray_model_space)
    }
}

pub struct SceneObjectBuilder {
    model: ModelInstance,
    rigid_body: RigidBodyInstance<f32>,
    transform: Transform3<f32>,
    bounds: Aabb<f32>,
}

impl SceneObjectBuilder {
    pub fn new(model: ModelInstance, rigid_body: RigidBodyInstance<f32>) -> Self {
        Self { 
            model,
            rigid_body,
            transform: Transform3::identity(),
            bounds: Aabb::new_empty() 
        }
    }

    pub fn with_transform(mut self, transform: &Transform3<f32>) -> Self {
        let old_bounds = self.model.bounds();
        let mut new_bounds = Aabb::new_empty();
        for i in 0..8 {
            let bounds_x = if i & 1 != 0 { old_bounds.bounds_max.x } else { old_bounds.bounds_min.x };
            let bounds_y = if i & 2 != 0 { old_bounds.bounds_max.y } else { old_bounds.bounds_min.y };
            let bounds_z = if i & 4 != 0 { old_bounds.bounds_max.z } else { old_bounds.bounds_min.z };
            let position = Vector3::new(bounds_x, bounds_y, bounds_z);
            let new_position = transform.transform_point(&position);

            new_bounds.grow(&new_position);
        }

        self.transform = *transform;
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

