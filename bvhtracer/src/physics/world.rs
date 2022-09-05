use super::rigid_body::*;
use super::force_registry::*;
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
    bodies: Vec<Rc<RefCell<RigidBody<S>>>>,
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

    pub fn start_frame(&mut self) {
        for body in self.bodies.iter_mut() {
            body.borrow_mut().clear_accumulators();
            body.borrow_mut().calculate_derived_data();
        }
    }

    fn integrate(&mut self, duration: S) {
        for body in self.bodies.iter_mut() {
            body.borrow_mut().integrate(duration);
        }
    }

    pub fn run_physics(&mut self, duration: S) {
        self.registry.apply_forces(duration);
        self.integrate(duration);
    }
}

