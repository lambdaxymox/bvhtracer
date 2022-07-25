extern crate bvhtracer;
extern crate cglinalg;

use bvhtracer::{
    ModelInstance,
    ModelBuilder,
    Normals,
    TextureCoordinates,
    MeshBuilder,
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

fn scene() -> ModelInstance {
    let top = top_triangle();
    let mesh = (0..100).fold(MeshBuilder::new(), |builder, i| {
            let displacement = Vector3::new(0_f32, 0_f32, i as f32);
            let primitive = Triangle::new(
                top.vertex0 - displacement,
                top.vertex1 - displacement,
                top.vertex2 - displacement,
            );
            let tex_coords = TextureCoordinates::default();
            let normals = Normals::default();

            builder.with_primitive(primitive, tex_coords, normals)
        })
        .build();
    let builder = ModelBuilder::new();
    
    builder.with_mesh(mesh).build()
}

#[test]
fn test_scene_stack_xy() {
    let scene = scene();
    let top = top_triangle();

    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex0.x == top.vertex0.x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex0.y == top.vertex0.y));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex1.x == top.vertex1.x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex1.y == top.vertex1.y));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex2.x == top.vertex2.x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertex2.y == top.vertex2.y));
}

/// Given a set of triangles that could intersect a ray, the intersection test
/// should return the closest one to the ray origin.
#[test]
fn test_intersection_should_return_closest_point_triangle() {
    let scene = scene();
    let closest = top_triangle();

    let ray_origin = Vector3::new(0_f32, 0_f32, 5_f32);
    let ray_direction = (closest.centroid() - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = scene.intersect(&ray);
    let expected = closest.intersect(&ray);

    assert!(result.is_some());
    assert!(expected.is_some());
    assert_eq!(result, expected);
}

#[test]
fn test_intersection_all_primitives_along_ray_should_hit() {
    let scene = scene();
    let closest = top_triangle();
    let ray_origin = Vector3::new(closest.centroid().x, closest.centroid().y, 5_f32);
    let ray_direction = (closest.centroid() - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let mesh = scene.model();
    let t_intersect_set = mesh.borrow().primitives().iter()
        .map(|triangle| { triangle.intersect(&ray) })
        .collect::<Vec<_>>();

    assert!(t_intersect_set.iter().all(|t_intersect| t_intersect.is_some()));
}

#[test]
fn test_intersection_closest_point_should_have_lowest_t() {
    let scene = scene();
    let closest = top_triangle();
    let ray_origin = Vector3::new(closest.centroid().x, closest.centroid().y, 5_f32);
    let ray_direction = (closest.centroid() - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = closest.intersect(&ray).unwrap();
    let mesh = scene.model();
    let t_intersect_set = mesh.borrow().primitives().iter()
        .map(|triangle| { triangle.intersect(&ray) })
        .collect::<Vec<_>>();
    let result = t_intersect_set.iter()
        .map(|elem| elem.unwrap())
        .fold(f32::MAX, |elem, acc| f32::min(acc, elem));

    assert_eq!(result, expected);
}

