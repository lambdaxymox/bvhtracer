extern crate bvhtracer;

use bvhtracer::{
    ModelInstance,
    ModelBuilder,
    Triangle,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};

fn scene() -> ModelInstance {
    let displacement = Vector3::new(5_f32, 0_f32, 0_f32);
    let triangle = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let triangle1 = Triangle::new(
        triangle.vertex0 - displacement,
        triangle.vertex1 - displacement,
        triangle.vertex2 - displacement,
    );
    let triangle2 = Triangle::new(
        triangle.vertex0 + displacement,
        triangle.vertex1 + displacement,
        triangle.vertex2 + displacement,
    );
    let primitives = vec![triangle1, triangle2];
    let builder = ModelBuilder::new();
    
    builder.with_primitives(primitives).build()
}


#[test]
fn test_two_triangles_intersection_hits1() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let mesh = scene.mesh();
    let target_origin = mesh.borrow().primitives()[0].centroid();
    let ray_direction = (target_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_two_triangles_intersection_hits2() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let mesh = scene.mesh();
    let target_origin = mesh.borrow().primitives()[1].centroid();
    let ray_direction = (target_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_some());
}

#[test]
fn test_two_triangles_intersection_miss() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let scene_origin = Vector3::zero();
    let ray_direction = (scene_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);

    assert!(result.is_none());
}

