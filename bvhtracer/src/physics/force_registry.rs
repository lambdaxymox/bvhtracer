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


#[derive(Clone, Debug, PartialEq)]
pub struct Instance<S> {
    inner: Rc<RefCell<RigidBody<S>>>,
}

impl<S> Instance<S> {
    pub (crate) fn rc(&self) -> Rc<RefCell<RigidBody<S>>> {
        self.inner.clone()
    }
}


struct ForceRegistryEntry<S> {
    body: Instance<S>,
    generator: Box<dyn ForceGenerator<S>>,
}

impl<S> ForceRegistryEntry<S> {
    fn new(body: Rc<RefCell<RigidBody<S>>>, generator: Box<dyn ForceGenerator<S>>) -> Self {
        let instance = Instance { inner: body, };

        Self { 
            body: instance, 
            generator, 
        }
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

    /*
    fn unregister(&mut self, body: Rc<RigidBody<S>>, generator: Box<dyn ForceGenerator<S>>) {
        unimplemented!()
    }
    */

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
            entry.generator.apply_force(&mut entry.body.inner.borrow_mut(), duration);
        }
    }
}

