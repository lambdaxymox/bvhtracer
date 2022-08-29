extern crate bvhtracer;
extern crate cglinalg;


use bvhtracer::{
    Triangle,
    Bvh,
    BvhBuilder,
};
use cglinalg::{
    Vector3,
};


fn bvh() -> (Bvh, Vec<Triangle<f32>>) {
    let displacement_x = Vector3::new(5_f32, 0_f32, 0_f32);
    let displacement_y = Vector3::new(0_f32, 5_f32, 0_f32);
    let triangle0 = Triangle::new(
        Vector3::new(0_f32, 1_f32 / 2_f32, 0_f32),
        Vector3::new(-1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
        Vector3::new(1_f32 / f32::sqrt(3_f32), -1_f32 / 2_f32, 0_f32),
    );
    let mut mesh = (-100..100).zip(-100..100).map(|(i, j)| {
        Triangle::new(
            triangle0.vertices[0] + (i as f32) * displacement_x + (j as f32) * displacement_y,
            triangle0.vertices[1] + (i as f32) * displacement_x + (j as f32) * displacement_y,
            triangle0.vertices[2] + (i as f32) * displacement_x + (j as f32) * displacement_y,
        )
    })
    .collect::<Vec<Triangle<_>>>();
    
    let builder = BvhBuilder::new();
    let bvh = builder.build_for(&mut mesh);

    (bvh, mesh)
}

fn animate(mesh: &mut [Triangle<f32>]) {
    let displacement0 = Vector3::new(0.3_f32, 0_f32, 0_f32);
    let displacement1 =  Vector3::new(0_f32, 0.3_f32, 0_f32);
    let displacement2 =  Vector3::new(0_f32, 0_f32, 0.3_f32);

    for triangle in mesh.iter_mut() {
        triangle.vertices[0] += displacement0;
        triangle.vertices[1] += displacement1;
        triangle.vertices[2] += displacement2;
    }
}

#[test]
fn test_bvh_refit_unchanged_on_same_mesh() {
    let (mut result, mesh) = bvh();
    let expected = result.clone();
    result.refit(&mesh);

    assert_eq!(result, expected);
}

#[test]
fn test_bvh_refit_idemotent() {
    let (mut bvh, mut mesh) = bvh();
    animate(&mut mesh);
    bvh.refit(&mesh);
    let expected = bvh.clone();
    bvh.refit(&mesh);
    let result = bvh.clone();

    assert_eq!(result, expected);
}

