use cglinalg::{
    Vector3, 
    SimdScalar,
    SimdScalarFloat,
};


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ray<S> 
where
    S: SimdScalar
{
    pub origin: Vector3<S>,
    pub direction: Vector3<S>,
    pub recip_direction: Vector3<S>,
    pub t: S,
}

impl<S> Ray<S> 
where
    S: SimdScalarFloat
{
    pub fn new(origin: Vector3<S>, direction: Vector3<S>, t: S) -> Self {
        let recip_direction = Vector3::new(
            S::one() / direction.x, 
            S::one() / direction.y, 
            S::one() / direction.z
        );

        Self { origin, direction, recip_direction, t, }
    }

    pub fn from_origin_dir(origin: Vector3<S>, direction: Vector3<S>) -> Self {
        Self::new(origin, direction, S::max_value())
    }
}

