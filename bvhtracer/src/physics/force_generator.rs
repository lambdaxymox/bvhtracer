use super::rigid_body::*;
use cglinalg::{
    SimdScalarFloat,
    Vector3,
    Magnitude,
};
use std::rc::{
    Rc,
};


pub trait ForceGenerator<S> {
    fn apply_force(&self, body: &mut RigidBody<S>, duration: S);
}


#[derive(Clone)]
pub struct Gravity<S> {
    gravity: Vector3<S>,
}

impl<S> Gravity<S>
where
    S: SimdScalarFloat,
{
    pub fn new(gravity: Vector3<S>) -> Self {
        Self { gravity, }
    }
}

impl<S> ForceGenerator<S> for Gravity<S>
where
    S: SimdScalarFloat,
{
    fn apply_force(&self, body: &mut RigidBody<S>, _duration: S) {
        if body.has_finite_mass() {
            let force = self.gravity * body.get_mass();
            body.apply_force(&force);
        }
    }
}

#[derive(Clone)]
pub struct Spring<S> {
    // Spring's body coordinates.
    spring_connection_point: Vector3<S>,
    // Object's body coordinates.
    object_connection_point: Vector3<S>,
    spring_constant: S,
    rest_length: S,
    object: Rc<RigidBody<S>>,
}

impl<S> Spring<S> {
    pub fn new(
        spring_connection_point_body: Vector3<S>,
        object: Rc<RigidBody<S>>,
        object_connection_point_body_object: Vector3<S>,
        spring_constant: S,
        rest_length: S,
    ) -> Self
    {
        Self {
            spring_connection_point: spring_connection_point_body,
            object_connection_point: object_connection_point_body_object,
            spring_constant,
            rest_length,
            object,
        }
    }
}

impl<S> ForceGenerator<S> for Spring<S>
where
    S: SimdScalarFloat,
{
    fn apply_force(&self, body: &mut RigidBody<S>, _duration: S) {
        // Calculate the two ends in world space.
        let lws = body.get_point_in_world_space(&self.spring_connection_point);
        let ows = self.object.get_point_in_world_space(&self.object_connection_point);
        let displacement = lws - ows;
        let force_magnitude = {
            let displacement_length = displacement.magnitude();
            let spring_displacement_from_rest = S::abs(displacement_length - self.rest_length);
            self.spring_constant * spring_displacement_from_rest
        };
        let force = {
            let displacement_direction = displacement.normalize();
            displacement_direction * (-force_magnitude)
        };
        body.apply_force_at_point(&force, &lws);
    }
}

