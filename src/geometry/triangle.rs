use crate::query::*;
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
}

impl<S> Triangle<S> 
where
    S: SimdScalarFloat
{
    pub fn new(vertex0: Vector3<S>, vertex1: Vector3<S>, vertex2: Vector3<S>) -> Self {
        Self { vertex0, vertex1, vertex2, }
    }

    pub fn centroid(&self) -> Vector3<S> {
        let one = S::one();
        let three = one + one + one;
        let one_third = one / three;
        
        (self.vertex0 + self.vertex1 + self.vertex2) * one_third
    }

    #[inline]
    pub fn intersect(&self, ray: &Ray<S>) -> Option<SurfaceInteraction<S>> {
        // Moeller-Trumbore ray/triangle intersection algorithm.
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
            let new_t = S::min(ray.t, t);

            Some(SurfaceInteraction::new(new_t, u, v))
        } else {
            None
        }
    }

    #[inline]
    pub fn intersect_mut(&self, intersection: &mut Intersection<S>) -> bool {
        // Moeller-Trumbore ray/triangle intersection algorithm
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;
        let normal = intersection.ray.direction.cross(&edge2);
        let area = edge1.dot(&normal);
        let threshold: S = num_traits::cast(0.0001_f64).unwrap();
        if S::abs(area) < threshold {
            // The ray is parallel to the triangle.
            return false;
        }
        let f = S::one() / area;
        let s = intersection.ray.origin - self.vertex0;
        let u = f * s.dot(&normal);
        if u < S::zero() || u > S::one() {
            return false;
        }
        let q = s.cross(&edge1);
        let v = f * intersection.ray.direction.dot(&q);
        if v < S::zero() || u + v > S::one() {
            return false;
        }
        let t = f * edge2.dot(&q);
        if t > threshold {
            intersection.ray.t = S::min(intersection.ray.t, t);
            intersection.interaction.t = intersection.ray.t;
            intersection.interaction.u = u;
            intersection.interaction.v = v;

            return true;
        } else {
            return false;
        }
    }
}

