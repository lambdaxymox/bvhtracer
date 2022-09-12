use crate::transform::*;
use cglinalg::{
    SimdScalarFloat,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransformComponent3<S> {
    transform: Transform3<S>,
    transform_inv: Transform3<S>,
}

impl<S> TransformComponent3<S>
where
    S: SimdScalarFloat,
{
    pub fn new(transform: Transform3<S>) -> Self {
        Self {
            transform: transform,
            transform_inv: transform.inverse().unwrap(),
        }
    }

    pub fn set_transform(&mut self, transform: &Transform3<S>) {
        self.transform = transform.clone();
        self.transform_inv = transform.inverse().unwrap();
    }

    #[inline]
    pub const fn transform(&self) -> &Transform3<S> {
        &self.transform
    }

    #[inline]
    pub const fn transform_inv(&self) -> &Transform3<S> {
        &self.transform_inv
    }
}

impl<S> Default for TransformComponent3<S>
where
    S: SimdScalarFloat,
{
    fn default() -> Self {
        Self {
            transform: Transform3::identity(),
            transform_inv: Transform3::identity(),
        }
    }
}

