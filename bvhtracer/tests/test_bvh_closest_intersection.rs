use bvhtracer::{
    ModelInstance,
    ModelBuilder,
    Normals,
    TextureCoordinates,
    MeshBuilder,
    SurfaceInteraction,
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
                top.vertices[0] - displacement,
                top.vertices[1] - displacement,
                top.vertices[2] - displacement,
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

    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[0].x == top.vertices[0].x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[0].y == top.vertices[0].y));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[1].x == top.vertices[1].x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[1].y == top.vertices[1].y));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[2].x == top.vertices[2].x));
    assert!(scene.model().borrow().primitives().iter().all(|triangle| triangle.vertices[2].y == top.vertices[2].y));
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
    let result = scene.intersect(&ray)
        .map(|intr| intr.interaction)
        .map(|surf| surf.t);
    let expected = closest.intersect(&ray)
        .map(|surf| surf.t);

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
    let intersection_set = mesh.borrow().primitives().iter()
        .map(|triangle| { triangle.intersect(&ray) })
        .collect::<Vec<_>>();
    let init = SurfaceInteraction::new(f32::MAX, f32::MAX, f32::MAX);
    let result = intersection_set.iter()
        .map(|elem| elem.unwrap())
        .fold(init, |elem, acc| {
            if acc.t < elem.t {
                acc
            } else { 
                elem
            }
        });

    assert_eq!(result, expected);
}

