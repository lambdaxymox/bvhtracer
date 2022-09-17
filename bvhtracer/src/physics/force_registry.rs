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


#[repr(transparent)]
#[derive(Clone, Debug, PartialEq)]
pub struct RigidBodyInstance<S> {
    inner: Rc<RefCell<RigidBody<S>>>,
}

impl<S> RigidBodyInstance<S> {
    pub const fn new(body: Rc<RefCell<RigidBody<S>>>) -> Self {
        Self {
            inner: body,
        }
    }

    pub (crate) fn rc(&self) -> Rc<RefCell<RigidBody<S>>> {
        self.inner.clone()
    }
}

struct ForceRegistryEntry<S> {
    body: RigidBodyInstance<S>,
    generator: Rc<dyn ForceGenerator<S>>,
}

impl<S> ForceRegistryEntry<S> {
    fn new(body: RigidBodyInstance<S>, generator: Rc<dyn ForceGenerator<S>>) -> Self {
        Self { body, generator, }
    }
}

pub struct ForceRegistry<S> {
    entries: Vec<ForceRegistryEntry<S>>,
}

impl<S> ForceRegistry<S> {
    pub const fn new() -> Self {
        Self {
            entries: vec![],
        }
    }

    pub fn register(&mut self, body: RigidBodyInstance<S>, generator: Rc<dyn ForceGenerator<S>>) {
        self.entries.push(ForceRegistryEntry::new(body, generator));
    }

    /*
    fn unregister(&mut self, body: Rc<RigidBody<S>>, generator: Box<dyn ForceGenerator<S>>) {
        unimplemented!()
    }
    */

    pub fn unregister_all(&mut self) {
        self.entries.clear();
    }
}

impl<S> ForceRegistry<S>
where
    S: SimdScalarFloat,
{
    pub fn apply_forces(&mut self, duration: S) {
        for entry in self.entries.iter_mut() {
            entry.generator.apply_force(&mut entry.body.inner.borrow_mut(), duration);
        }
    }
}

