use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Triangle {
    pub vertex0: Vector3<f32>,
    pub vertex1: Vector3<f32>,
    pub vertex2: Vector3<f32>,
    pub centroid: Vector3<f32>,
}

impl Triangle {
    pub fn new(vertex0: Vector3<f32>, vertex1: Vector3<f32>, vertex2: Vector3<f32>) -> Self {
        let one_third = 1_f32 / 3_f32;
        let centroid = (vertex0 + vertex1 + vertex2) * one_third;

        Self { vertex0, vertex1, vertex2, centroid, }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Ray> {
        let edge1 = self.vertex1 - self.vertex0;
        let edge2 = self.vertex2 - self.vertex0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);
        if a > -0.0001 && a < 0.0001 {
            // The ray is parallel to the triangle.
            // return *ray;
            return None;
        }
        let f = 1_f32 / a;
        let s = ray.origin - self.vertex0;
        let u = f * s.dot(&h);
        if u < 0_f32 || u > 1_f32 {
            // return *ray;
            return None;
        }
        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);
        if v < 0_f32 || u + v > 1_f32 {
            // return *ray;
            return None;
        }
        let t = f * edge2.dot(&q);
        if t > 0.0001 {
            let t_intersect = f32::min(ray.t, t);
    
            return Some(Ray::new(ray.origin, ray.direction, t_intersect));
        } else {
            return None;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub recip_direction: Vector3<f32>,
    pub t: f32,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>, t: f32) -> Self {
        let recip_direction = Vector3::new(
            1_f32 / direction.x, 
            1_f32 / direction.y, 
            1_f32 / direction.z
        );

        Self { origin, direction, recip_direction, t, }
    }

    pub fn from_origin_dir(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self::new(origin, direction, f32::MAX)
    }
}

