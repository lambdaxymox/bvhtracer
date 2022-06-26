use cglinalg::{
    Vector3,
    SimdScalar,
    SimdScalarFloat,
};
use crate::ray::*;


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Aabb<S> 
where
    S: SimdScalar
{
    pub box_min: Vector3<S>,
    pub box_max: Vector3<S>,
}

impl<S> Aabb<S> 
where
    S: SimdScalarFloat
{
    pub fn new(b_min: Vector3<S>, b_max: Vector3<S>) -> Self {
        Self { box_min: b_min, box_max: b_max, }
    }

    pub fn grow(&mut self, position: &Vector3<S>) {
        #[inline]
        fn min<S: SimdScalarFloat>(vector1: &Vector3<S>, vector2: &Vector3<S>) -> Vector3<S> {
            Vector3::new(
                S::min(vector1.x, vector2.x),
                S::min(vector1.y, vector2.y),
                S::min(vector1.z, vector2.z),
            )
        }

        #[inline]
        fn max<S: SimdScalarFloat>(vector1: &Vector3<S>, vector2: &Vector3<S>) -> Vector3<S> {
            Vector3::new(
                S::max(vector1.x, vector2.x),
                S::max(vector1.y, vector2.y),
                S::max(vector1.z, vector2.z),
            )
        }

        self.box_min = min(&self.box_min, position);
        self.box_max = max(&self.box_max, position);
    }

    pub fn area(&self) -> S {
        let extent = self.extent();

        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }

    #[inline]
    pub fn extent(&self) -> Vector3<S> {
        self.box_max - self.box_min
    }

    pub fn intersect(&self, ray: &Ray<S>) -> Option<S> {
        let t_x1 = (self.box_min.x - ray.origin.x) * ray.recip_direction.x;
        let t_x2 = (self.box_max.x - ray.origin.x) * ray.recip_direction.x;
        let t_min = S::min(t_x1, t_x2);
        let t_max = S::max(t_x1, t_x2);
        let t_y1 = (self.box_min.y - ray.origin.y) * ray.recip_direction.y; 
        let t_y2 = (self.box_max.y - ray.origin.y) * ray.recip_direction.y;
        let t_min = S::max(t_min, S::min(t_y1, t_y2)); 
        let t_max = S::min(t_max, S::max(t_y1, t_y2));
        let t_z1 = (self.box_min.z - ray.origin.z) * ray.recip_direction.z;
        let t_z2 = (self.box_max.z - ray.origin.z) * ray.recip_direction.z;
        let t_min = S::max(t_min, S::min(t_z1, t_z2)); 
        let t_max = S::min(t_max, S::max(t_z1, t_z2));
        
        if (t_max >= t_min) && (t_min < ray.t) && (t_max > S::zero()) {
            Some(t_min)
        } else {
            None
        }
    }
}

