extern crate bvhtracer;

use bvhtracer::{
    Scene,
    SceneBuilder,
    Triangle,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};

fn scene() -> Scene {
    let triangle1 = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let triangle2 = Triangle::new(
        Vector3::new(2_f32 / f32::sqrt(3_f32), 1_f32, -0.2_f32),
        Vector3::new(-2_f32 / f32::sqrt(3_f32), 1_f32, -0.2_f32),
        Vector3::new(0_f32, -1_f32, -0.2_f32),
    );
    let triangles = vec![triangle1, triangle2];
    let builder = SceneBuilder::new();
    
    builder.with_objects(triangles).build()
}

#[test]
fn test_one_triangle_should_have_one_volume() {
    let scene = scene();

    assert_eq!(scene.bvh.nodes_used(), 1);
}

#[test]
fn test_one_triangle_intersection_hits1() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let target_origin = scene.objects[0].centroid;
    let ray_direction = (target_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_one_triangle_intersection_hits2() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let target_origin = scene.objects[1].centroid;
    let ray_direction = (target_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_one_triangle_intersection_miss() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_none());
}

