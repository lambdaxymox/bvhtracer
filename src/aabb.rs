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
    pub bounds_min: Vector3<S>,
    pub bounds_max: Vector3<S>,
}

impl<S> Aabb<S> 
where
    S: SimdScalarFloat
{
    pub fn new(b_min: Vector3<S>, b_max: Vector3<S>) -> Self {
        Self { bounds_min: b_min, bounds_max: b_max, }
    }

    pub fn grow(&mut self, position: &Vector3<S>) {
        // NOTE: We use local implementations of min and max of vector components here because the
        // compiler does not seem to want to inline it here.
        #[inline] fn __min<S: SimdScalarFloat>(this: &Vector3<S>, that: &Vector3<S>) -> Vector3<S> { 
            // this.component_min(that)
            Vector3::new(
                S::min(this.x, that.x),
                S::min(this.y, that.y),
                S::min(this.z, that.z),
            )
        }

        #[inline] fn __max<S: SimdScalarFloat>(this: &Vector3<S>, that: &Vector3<S>) -> Vector3<S> {
            // this.component_max(that)
            Vector3::new(
                S::max(this.x, that.x),
                S::max(this.y, that.y),
                S::max(this.z, that.z),
            )
        }

        self.bounds_min = __min(&self.bounds_min, position);
        self.bounds_max = __max(&self.bounds_max, position);
    }

    pub fn grow_aabb(&mut self, new_bounding_box: &Aabb<S>) { 
        if new_bounding_box.bounds_min.x != S::max_value() { 
            self.grow( &new_bounding_box.bounds_min); 
            self.grow( &new_bounding_box.bounds_max); 
        } 
    }

    pub fn area(&self) -> S {
        let extent = self.extent();

        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }

    #[inline]
    pub fn extent(&self) -> Vector3<S> {
        self.bounds_max - self.bounds_min
    }

    pub fn centroid(&self) -> Vector3<S> {
        let one_half = num_traits::cast(0.5_f64).unwrap();

        (self.bounds_min + self.bounds_max) * one_half
    }

    pub fn intersect(&self, ray: &Ray<S>) -> Option<S> {
        let t_x1 = (self.bounds_min.x - ray.origin.x) * ray.recip_direction.x;
        let t_x2 = (self.bounds_max.x - ray.origin.x) * ray.recip_direction.x;
        let t_min = S::min(t_x1, t_x2);
        let t_max = S::max(t_x1, t_x2);
        let t_y1 = (self.bounds_min.y - ray.origin.y) * ray.recip_direction.y; 
        let t_y2 = (self.bounds_max.y - ray.origin.y) * ray.recip_direction.y;
        let t_min = S::max(t_min, S::min(t_y1, t_y2)); 
        let t_max = S::min(t_max, S::max(t_y1, t_y2));
        let t_z1 = (self.bounds_min.z - ray.origin.z) * ray.recip_direction.z;
        let t_z2 = (self.bounds_max.z - ray.origin.z) * ray.recip_direction.z;
        let t_min = S::max(t_min, S::min(t_z1, t_z2)); 
        let t_max = S::min(t_max, S::max(t_z1, t_z2));
        
        if (t_max >= t_min) && (t_min < ray.t) && (t_max > S::zero()) {
            Some(t_min)
        } else {
            None
        }
    }
}

