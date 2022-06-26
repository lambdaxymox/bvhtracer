use cglinalg::{
    Vector3,
    SimdScalarFloat,
};
use crate::triangle::*;
use crate::ray::*;


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Aabb {
    pub b_min: Vector3<f32>,
    pub b_max: Vector3<f32>,
}

impl Aabb {
    pub fn new(b_min: Vector3<f32>, b_max: Vector3<f32>) -> Self {
        Self { b_min, b_max, }
    }

    pub fn grow(&mut self, position: &Vector3<f32>) {
        #[inline]
        fn min(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::min(vector1.x, vector2.x),
                f32::min(vector1.y, vector2.y),
                f32::min(vector1.z, vector2.z),
            )
        }

        #[inline]
        fn max(vector1: &Vector3<f32>, vector2: &Vector3<f32>) -> Vector3<f32> {
            Vector3::new(
                f32::max(vector1.x, vector2.x),
                f32::max(vector1.y, vector2.y),
                f32::max(vector1.z, vector2.z),
            )
        }

        self.b_min = min(&self.b_min, position);
        self.b_max = max(&self.b_max, position);
    }

    pub fn area(&self) -> f32 {
        let extent = self.extent();

        extent.x * extent.y + extent.y * extent.z + extent.z * extent.x
    }

    #[inline]
    pub fn extent(&self) -> Vector3<f32> {
        self.b_max - self.b_min
    }

    pub fn intersect(&self, ray: &Ray<f32>) -> f32 {
        let t_x1 = (self.b_min.x - ray.origin.x) * ray.recip_direction.x;
        let t_x2 = (self.b_max.x - ray.origin.x) * ray.recip_direction.x;
        let t_min = f32::min(t_x1, t_x2);
        let t_max = f32::max(t_x1, t_x2);
        let t_y1 = (self.b_min.y - ray.origin.y) * ray.recip_direction.y; 
        let t_y2 = (self.b_max.y - ray.origin.y) * ray.recip_direction.y;
        let t_min = f32::max(t_min, f32::min(t_y1, t_y2)); 
        let t_max = f32::min(t_max, f32::max(t_y1, t_y2));
        let t_z1 = (self.b_min.z - ray.origin.z) * ray.recip_direction.z;
        let t_z2 = (self.b_max.z - ray.origin.z) * ray.recip_direction.z;
        let t_min = f32::max(t_min, f32::min(t_z1, t_z2)); 
        let t_max = f32::min(t_max, f32::max(t_z1, t_z2));
        
        if (t_max >= t_min) && (t_min < ray.t) && (t_max > 0_f32) {
            t_min
        } else {
            f32::MAX
        }
    }
}

