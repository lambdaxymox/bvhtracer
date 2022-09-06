use super::rigid_body::*;
use super::force_registry::*;
use super::force_generator::*;
use cglinalg::{
    SimdScalarFloat,
};
use std::cell::{
    RefCell,
};
use std::rc::{
    Rc,
};


pub struct World<S> {
    bodies: Vec<RigidBodyInstance<S>>,
    registry: ForceRegistry<S>,
}

impl<S> World<S>
where
    S: SimdScalarFloat,
{
    pub fn new() -> Self {
        Self {
            bodies: vec![],
            registry: ForceRegistry::new(),
        }
    }

    pub fn register_body(&mut self, body: RigidBody<S>) -> RigidBodyInstance<S> {
        let body_instance = Rc::new(RefCell::new(body));
        let instance = RigidBodyInstance::new(body_instance);
        self.bodies.push(instance.clone());

        instance
    }

    pub fn register_force_generator(&mut self, body: RigidBodyInstance<S>, generator: Rc<dyn ForceGenerator<S>>) {
        self.registry.register(body, generator);
    }

    pub fn start_frame(&mut self) {
        for body in self.bodies.iter_mut() {
            body.rc().borrow_mut().clear_accumulators();
            body.rc().borrow_mut().calculate_derived_data();
        }
    }

    fn integrate(&mut self, duration: S) {
        for body in self.bodies.iter_mut() {
            body.rc().borrow_mut().integrate(duration);
        }
    }

    pub fn run_physics(&mut self, duration: S) {
        self.registry.apply_forces(duration);
        self.integrate(duration);
    }
}

