use crate::transform::*;
use cglinalg::{
    SimdScalarFloat,
};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TransformComponent3<S> {
    transform: Transform3<S>,
    transform_inverse: Transform3<S>,
}

impl<S> TransformComponent3<S>
where
    S: SimdScalarFloat,
{
    pub fn new(transform: Transform3<S>) -> Self {
        Self {
            transform: transform,
            transform_inverse: transform.inverse().unwrap(),
        }
    }

    pub fn set_transform(&mut self, transform: &Transform3<S>) {
        self.transform = transform.clone();
        self.transform_inverse = transform.inverse().unwrap();
    }

    #[inline]
    pub const fn transform(&self) -> &Transform3<S> {
        &self.transform
    }

    #[inline]
    pub const fn transform_inv(&self) -> &Transform3<S> {
        &self.transform_inverse
    }
}

impl<S> Default for TransformComponent3<S>
where
    S: SimdScalarFloat,
{
    fn default() -> Self {
        Self {
            transform: Transform3::default(),
            transform_inverse: Transform3::default(),
        }
    }
}

