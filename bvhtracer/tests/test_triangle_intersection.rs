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
fn test_triangle_intersection_vertex_hits1() {
    let scene = scene();
    let vertex = Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_triangle_intersection_vertex_hits2() {
    let scene = scene();
    let vertex = Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_triangle_intersection_vertex_hits3() {
    let scene = scene();
    let vertex = Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
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
    let result_t = scene.intersect(&ray).unwrap().t;
    let result = Ray::new(ray.origin, ray.direction, result_t);

    assert_eq!(result, expected);
}

#[test]
fn test_triangle_intersection_vertex1() {
    let scene = scene();
    let vertex = Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected_t = f32::sqrt(101_f32 / 4_f32);
    let expected = Ray::new(ray_origin, ray_direction, expected_t);
    let result_t = scene.intersect(&ray).unwrap().t;
    let result = Ray::new(ray.origin, ray.direction, result_t);

    assert_eq!(result, expected);
}

#[test]
fn test_triangle_intersection_vertex2() {
    let scene = scene();
    let vertex = Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected_t = f32::sqrt(307_f32 / 12_f32);
    let expected = Ray::new(ray_origin, ray_direction, expected_t);
    let result_t = scene.intersect(&ray).unwrap().t;
    let result = Ray::new(ray.origin, ray.direction, result_t);

    assert_eq!(result, expected);
}

#[test]
fn test_triangle_intersection_vertex3() {
    let scene = scene();
    let vertex = Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (vertex - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected_t = f32::sqrt(307_f32 / 12_f32);
    let expected = Ray::new(ray_origin, ray_direction, expected_t);
    let result_t = scene.intersect(&ray).unwrap().t;
    let result = Ray::new(ray.origin, ray.direction, result_t);

    assert_eq!(result, expected);
}

#[test]
fn test_triangle_intersection_vertex_miss1() {
    let scene = scene();
    let vertex = Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32);
    let displacement = Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32);
    let target = vertex + displacement;
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_none());
}

#[test]
fn test_triangle_intersection_vertex_miss2() {
    let scene = scene();
    let vertex = Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let displacement = Vector3::new(0_f32, -1_f32 / 2_f32, 0_f32);
    let target = vertex + displacement;
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_none());
}

#[test]
fn test_triangle_intersection_vertex_miss3() {
    let scene = scene();
    let vertex = Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32);
    let displacement = Vector3::new(0_f32, -1_f32 / 2_f32, 0_f32);
    let target = vertex + displacement;
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_none());
}

#[test]
fn test_triangle_closest_t1() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray_t = f32::MAX;
    let ray = Ray::new(ray_origin, ray_direction, ray_t);
    let result = scene.intersect(&ray).unwrap();

    assert!(result.t <= ray_t);
}

#[test]
fn test_triangle_closest_t2() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray_t = 100_f32;
    let ray = Ray::new(ray_origin, ray_direction, ray_t);
    let result = scene.intersect(&ray).unwrap();

    assert!(result.t <= ray_t);
}


#[test]
fn test_triangle_closest_t3() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray_t = 0.01_f32;
    let ray = Ray::new(ray_origin, ray_direction, ray_t);
    let result = scene.intersect(&ray).unwrap();

    assert!(result.t <= ray_t);
}

