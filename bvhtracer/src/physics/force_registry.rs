use super::rigid_body::*;
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


struct ForceRegistryEntry<S> {
    body: Rc<RefCell<RigidBody<S>>>,
    generator: Box<dyn ForceGenerator<S>>,
}

impl<S> ForceRegistryEntry<S> {
    fn new(body: Rc<RefCell<RigidBody<S>>>, generator: Box<dyn ForceGenerator<S>>) -> Self {
        Self { body, generator, }
    }
}

pub struct ForceRegistry<S> {
    entries: Vec<ForceRegistryEntry<S>>,
}

impl<S> ForceRegistry<S> {
    pub fn new() -> Self {
        Self {
            entries: vec![],
        }
    }

    fn register(&mut self, body: Rc<RefCell<RigidBody<S>>>, generator: Box<dyn ForceGenerator<S>>) {
        self.entries.push(ForceRegistryEntry::new(body, generator));
    }

    fn unregister(&mut self, body: Rc<RigidBody<S>>, generator: Box<dyn ForceGenerator<S>>) {
        unimplemented!()
    }

    fn unregister_all(&mut self) {
        self.entries.clear();
    }
}

impl<S> ForceRegistry<S>
where
    S: SimdScalarFloat,
{
    pub fn apply_forces(&mut self, duration: S) {
        for entry in self.entries.iter_mut() {
            entry.generator.apply_force(&mut entry.body.borrow_mut(), duration);
        }
    }
}

