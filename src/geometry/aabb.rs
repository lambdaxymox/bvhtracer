use cglinalg::{
    Vector3,
    SimdScalar,
    SimdScalarFloat,
};
use crate::query::{
    Ray,
};


#[repr(C)]
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
    pub fn new(bounds_min: Vector3<S>, bounds_max: Vector3<S>) -> Self {
        Self { bounds_min, bounds_max, }
    }

    pub fn new_empty() -> Self {
        Self { 
            bounds_min: Vector3::from_fill(S::max_value()), 
            bounds_max: Vector3::from_fill(-S::max_value()) 
        }
    }

    pub fn grow(&mut self, position: &Vector3<S>) {
        self.bounds_min = Vector3::component_min(&self.bounds_min, position);
        self.bounds_max = Vector3::component_max(&self.bounds_max, position);
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

