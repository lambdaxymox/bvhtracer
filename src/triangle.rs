use crate::ray::*;
use cglinalg::{
    Vector3,
    SimdScalar,
    SimdScalarFloat,
};


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Triangle<S> 
where
    S: SimdScalar
{
    pub vertex0: Vector3<S>,
    pub vertex1: Vector3<S>,
    pub vertex2: Vector3<S>,
    pub centroid: Vector3<S>,
}

impl<S> Triangle<S> 
where
    S: SimdScalarFloat
{
    pub fn new(vertex0: Vector3<S>, vertex1: Vector3<S>, vertex2: Vector3<S>) -> Self {
        let one = S::one();
        let three = one + one + one;
        let one_third = one / three;
        let centroid = (vertex0 + vertex1 + vertex2) * one_third;

        Self { vertex0, vertex1, vertex2, centroid, }
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<S>) -> Option<S> {
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;
        let normal = ray.direction.cross(&edge2);
        let area = edge1.dot(&normal);
        let threshold: S = num_traits::cast(0.0001_f64).unwrap();
        if S::abs(area) < threshold {
            // The ray is parallel to the triangle.
            return None;
        }
        let f = S::one() / area;
        let s = ray.origin - self.vertex0;
        let u = f * s.dot(&normal);
        if u < S::zero() || u > S::one() {
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < S::zero() || u + v > S::one() {
            return None;
        }
        let t = f * edge2.dot(&q);
        if t > threshold {
            Some(S::min(ray.t, t))
        } else {
            None
        }
    }

    #[inline]
    pub fn intersect_mut(&self, ray: &mut Ray<S>) -> bool {
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;
        let normal = ray.direction.cross(&edge2);
        let area = edge1.dot(&normal);
        let threshold: S = num_traits::cast(0.0001_f64).unwrap();
        if S::abs(area) < threshold {
            // The ray is parallel to the triangle.
            return false;
        }
        let f = S::one() / area;
        let s = ray.origin - self.vertex0;
        let u = f * s.dot(&normal);
        if u < S::zero() || u > S::one() {
            return false;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < S::zero() || u + v > S::one() {
            return false;
        }
        let t = f * edge2.dot(&q);
        if t > threshold {
            ray.t = S::min(ray.t, t);

            return true;
        } else {
            return false;
        }
    }
}

