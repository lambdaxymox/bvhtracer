use super::rigid_body::*;
use super::force_registry::*;
use cglinalg::{
    SimdScalarFloat,
    Vector3,
    Magnitude,
};
use std::rc::{
    Rc,
};
use std::fmt;
use std::marker;


pub trait ForceGenerator<S>: fmt::Debug {
    fn apply_force(&self, body: &mut RigidBody<S>, duration: S);
}

#[derive(Clone, Debug)]
pub struct NullForce<S> {
    _marker: marker::PhantomData<S>,
}

impl<S> NullForce<S>
where
    S: SimdScalarFloat,
{
    pub fn new() -> Self { 
        Self {
            _marker: marker::PhantomData,
        }
    }
}

impl<S> ForceGenerator<S> for NullForce<S>
where
    S: SimdScalarFloat,
{
    fn apply_force(&self, _body: &mut RigidBody<S>, _duration: S) { 
    }
}


#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Spring<S> {
    object: RigidBodyInstance<S>,
    // Spring's body coordinates.
    spring_connection_point: Vector3<S>,
    // Object's body coordinates.
    object_connection_point: Vector3<S>,
    spring_constant: S,
    rest_length: S,
}

impl<S> Spring<S> {
    pub fn new(
        object: RigidBodyInstance<S>,
        spring_connection_point_body: Vector3<S>,
        object_connection_point_body_object: Vector3<S>,
        spring_constant: S,
        rest_length: S,
    ) -> Self
    {
        Self {
            object,
            spring_connection_point: spring_connection_point_body,
            object_connection_point: object_connection_point_body_object,
            spring_constant,
            rest_length,
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
        let ows = self.object
            .rc()
            .borrow()
            .get_point_in_world_space(&self.object_connection_point);
        let displacement = lws - ows;
        
        eprintln!("self.spring_connection_point = {:?}", self.spring_connection_point);
        eprintln!("self.object_connection_point = {:?}", self.object_connection_point);
        eprintln!("lws = {:?}", lws);
        eprintln!("ows = {:?}", ows);
        eprintln!("displacement = {:?}", displacement);
        
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

