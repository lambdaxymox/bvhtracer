extern crate bvhtracer;


use bvhtracer::{
    Transform3,
};
use cglinalg::{
    Degrees,
    Vector3,
    Unit,
};


#[test]
fn test_transform_composition1() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_translation(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.to_matrix4x4() * transform2.to_matrix4x4();
    let result = (transform1 * transform2).to_matrix4x4();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition2() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_scale(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.to_matrix4x4() * transform2.to_matrix4x4();
    let result = (transform1 * transform2).to_matrix4x4();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition3() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(45_f64)
    );
    let expected = transform1.to_matrix4x4() * transform2.to_matrix4x4();
    let result = (transform1 * transform2).to_matrix4x4();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition4() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(45_f64)
    );
    let transform3 = Transform3::from_scale(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.to_matrix4x4() * transform2.to_matrix4x4() * transform3.to_matrix4x4();
    let result = (transform1 * transform2 * transform3).to_matrix4x4();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition5() {
    let transform1 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(45_f64)
    );
    let transform2 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(30_f64)
    );
    let expected = transform1.to_matrix4x4() * transform2.to_matrix4x4();
    let result = (transform1 * transform2).to_matrix4x4();

    assert_eq!(result, expected);
}
