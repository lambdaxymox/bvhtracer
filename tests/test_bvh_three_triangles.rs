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
    let displacement = Vector3::new(0_f32, 10_f32, 0_f32);
    let triangle1 = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let triangle2 = Triangle::new(
        triangle1.vertex0 - displacement,
        triangle1.vertex1 - displacement,
        triangle1.vertex2 - displacement
    );
    let triangle3 = Triangle::new(
        triangle1.vertex0 + displacement,
        triangle1.vertex1 + displacement,
        triangle1.vertex2 + displacement
    );
    let triangles = vec![triangle1, triangle2, triangle3];
    let builder = SceneBuilder::new();
    
    builder.with_objects(triangles).build()
}

#[test]
fn test_three_triangles_centroids_hit() {
    let scene = scene();
    for triangle in scene.objects.iter() {
        let ray_origin = Vector3::new(0_f32, 0_f32, 10_f32);
        let ray_direction = (triangle.centroid - ray_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(triangle.intersect(&ray).is_some());
    }
}
