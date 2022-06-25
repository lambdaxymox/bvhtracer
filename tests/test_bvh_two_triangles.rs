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

