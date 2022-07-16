extern crate bvhtracer;
extern crate cglinalg;

use bvhtracer::{
    Triangle,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};


fn scene() -> Triangle<f32> {
    Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    )
}

#[test]
fn test_triangle_intersection_hits() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_triangle_intersection_center() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected_t = 5_f32;
    let expected = Ray::new(ray_origin, ray_direction, expected_t);
    let result_t = scene.intersect(&ray).unwrap();
    let result = Ray::new(ray.origin, ray.direction, result_t);

    assert_eq!(result, expected);
}

