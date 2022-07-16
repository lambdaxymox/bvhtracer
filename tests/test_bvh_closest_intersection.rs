extern crate bvhtracer;
extern crate cglinalg;

use bvhtracer::{
    Model,
    ModelBuilder,
    Triangle,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};


fn top_triangle() -> Triangle<f32> {
    Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    )
}

fn scene() -> Model {
    let triangle = top_triangle();
    let displacement = Vector3::new(0_f32, 0_f32, 1_f32);
    let mesh = (0..100).map(|i| {
            Triangle::new(
                triangle.vertex0 - (i as f32) * displacement,
                triangle.vertex1 - (i as f32) * displacement,
                triangle.vertex2 - (i as f32) * displacement,            
            )
        })
        .rev()
        .collect::<Vec<_>>();
    let builder = ModelBuilder::new();
    
    builder.with_mesh(mesh).build()
}


/// Given a set of triangles that could intersect a ray, the intersection test
/// should return the closest one to the ray origin.
#[test]
fn test_intersection_should_return_closest_point_triangle() {
    let scene = scene();
    let closest = top_triangle();

    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (closest.centroid - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);
    let expected = closest.intersect(&ray);

    assert!(result.is_some());
    assert!(expected.is_some());
    assert_eq!(result, expected);
}

