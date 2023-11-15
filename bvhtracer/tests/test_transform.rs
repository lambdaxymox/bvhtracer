use bvhtracer::{
    Transform3,
};
use cglinalg::{
    Degrees,
    Vector3,
    Unit,
    Radians,
    Rotation3,
    Matrix4x4,
};
use approx::{
    assert_relative_eq,
};
use std::f64;


#[test]
fn test_transform_composition1() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_translation(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.compute_matrix() * transform2.compute_matrix();
    let result = (transform1 * transform2).compute_matrix();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition2() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_nonuniform_scale(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.compute_matrix() * transform2.compute_matrix();
    let result = (transform1 * transform2).compute_matrix();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition3() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(45_f64)
    );
    let expected = transform1.compute_matrix() * transform2.compute_matrix();
    let result = (transform1 * transform2).compute_matrix();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_composition4() {
    let transform1 = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 2_f64));
    let transform2 = Transform3::from_axis_angle(
        &Unit::from_value(Vector3::new(1_f64, -1_f64, 1_f64)), 
        Degrees(45_f64)
    );
    let transform3 = Transform3::from_nonuniform_scale(&Vector3::new(4_f64, 5_f64, 6_f64));
    let expected = transform1.compute_matrix() * transform2.compute_matrix() * transform3.compute_matrix();
    let result = (transform1 * transform2 * transform3).compute_matrix();

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
    let expected = transform1.compute_matrix() * transform2.compute_matrix();
    let result = (transform1 * transform2).compute_matrix();

    assert_eq!(result, expected);
}

#[test]
fn test_transform_rotation1() {
    let angle = Radians(0_f64);
    let transform = Transform3::from_angle_z(angle);
    let vector = Vector3::unit_x();
    let expected = Vector3::unit_x();
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_rotation2() {
    let angle = Radians(f64::consts::FRAC_PI_2);
    let transform = Transform3::from_angle_z(angle);
    let vector = Vector3::unit_x();
    let expected = Vector3::unit_y();
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_rotation3() {
    let angle = Radians(f64::consts::PI);
    let transform = Transform3::from_angle_z(angle);
    let vector = Vector3::unit_x();
    let expected = -Vector3::unit_x();
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_rotation4() {
    let angle = Radians(3_f64 * f64::consts::PI / 2_f64);
    let transform = Transform3::from_angle_z(angle);
    let vector = Vector3::unit_x();
    let expected = -Vector3::unit_y();
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_rotation5() {
    let angle = Radians(2_f64 * f64::consts::PI);
    let transform = Transform3::from_angle_z(angle);
    let vector = Vector3::unit_x();
    let expected = Vector3::unit_x();
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_translation_point1() {
    let transform = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 3_f64));
    let point = Vector3::new(-1_f64, -1_f64, 4_f64);
    let expected = Vector3::new(0_f64, 1_f64, 7_f64);
    let result = transform.transform_point(&point);

    assert_eq!(result, expected);
}

#[test]
fn test_translation_point2() {
    let transform = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 3_f64));
    let point = Vector3::new(-1_f64, -1_f64, 4_f64);
    let expected = Vector3::new(-2_f64, -3_f64, 1_f64);
    let result = transform.inverse_transform_point(&point);

    assert_eq!(result, expected);
}

#[test]
fn test_translation_vector1() {
    let transform = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 3_f64));
    let vector = Vector3::new(-1_f64, -1_f64, 4_f64);
    let expected = vector;
    let result = transform.transform_vector(&vector);

    assert_eq!(result, expected);
}

#[test]
fn test_translation_vector2() {
    let transform = Transform3::from_translation(&Vector3::new(1_f64, 2_f64, 3_f64));
    let vector = Vector3::new(-1_f64, -1_f64, 4_f64);
    let expected = vector;
    let result = transform.inverse_transform_vector(&vector);

    assert_eq!(result, expected);
}

#[test]
fn test_scale_point1() {
    let scale = 4_f64;
    let transform = Transform3::from_scale(scale);
    let point = Vector3::new(2_f64, 4_f64, 8_f64);
    let expected = Vector3::new(8_f64, 16_f64, 32_f64);
    let result = transform.transform_point(&point);

    assert_eq!(result, expected);
}

#[test]
fn test_scale_vector1() {
    let scale = 4_f64;
    let transform = Transform3::from_scale(scale);
    let vector = Vector3::new(2_f64, 4_f64, 8_f64);
    let expected = Vector3::new(8_f64, 16_f64, 32_f64);
    let result = transform.transform_vector(&vector);

    assert_eq!(result, expected);
}

#[test]
fn test_inverse_scale_point1() {
    let scale = 4_f64;
    let transform = Transform3::from_scale(scale);
    let point = Vector3::new(2_f64, 4_f64, 8_f64);
    let expected = Vector3::new(2_f64 / scale, 4_f64 / scale, 8_f64 / scale);
    let result = transform.inverse_transform_point(&point);

    assert_eq!(result, expected);
}

#[test]
fn test_inverse_scale_vector1() {
    let scale = 4_f64;
    let transform = Transform3::from_scale(scale);
    let vector = Vector3::new(2_f64, 4_f64, 8_f64);
    let expected = Vector3::new(2_f64 / scale, 4_f64 / scale, 8_f64 / scale);
    let result = transform.inverse_transform_vector(&vector);

    assert_eq!(result, expected);
}

#[test]
fn test_translation_scale_point() {
    let scale = Vector3::new(1_f64, 2_f64, 3_f64);
    let translation = Vector3::new(2_f64, 3_f64, 4_f64);
    let transform = Transform3::from_scale_translation(&scale, &translation);
    let point = Vector3::new(5_f64, 2_f64, 3_f64);
    let expected = Vector3::new(7_f64, 7_f64, 13_f64);
    let result = transform.transform_point(&point);

    assert_eq!(result, expected);
}

#[test]
fn test_translation_scale_vector() {
    let scale = Vector3::new(1_f64, 2_f64, 3_f64);
    let translation = Vector3::new(2_f64, 3_f64, 4_f64);
    let transform = Transform3::from_scale_translation(&scale, &translation);
    let vector = Vector3::new(5_f64, 2_f64, 3_f64);
    let expected = Vector3::new(5_f64, 4_f64, 9_f64);
    let result = transform.transform_vector(&vector);

    assert_eq!(result, expected);
}

#[test]
fn test_transform_point1() {
    let scale = Vector3::new(2_f64, 3_f64, 4_f64);
    let axis = &Unit::from_value(Vector3::unit_z());
    let angle = Radians(f64::consts::FRAC_PI_2);
    let rotation = Rotation3::from_axis_angle(&axis, angle);
    let translation = Vector3::new(0_f64, 3_f64, 0_f64);
    let transform = Transform3::new(&scale, &translation, rotation);
    let transform1 = Transform3::from_scale_axis_angle(&scale, &axis, angle);
    let point = Vector3::unit_x();
    eprintln!("{}", transform1.transform_point(&point));
    let expected = Vector3::new(0_f64, 5_f64, 0_f64);
    let result = transform.transform_point(&point);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_vector1() {
    let scale = Vector3::new(2_f64, 3_f64, 4_f64);
    let axis = &Unit::from_value(Vector3::unit_z());
    let angle = Radians(f64::consts::FRAC_PI_2);
    let rotation = Rotation3::from_axis_angle(&axis, angle);
    let translation = Vector3::new(0_f64, 3_f64, 0_f64);
    let transform = Transform3::new(&scale, &translation, rotation);
    let vector = Vector3::unit_x();
    let expected = Vector3::new(0_f64, 2_f64, 0_f64);
    let result = transform.transform_vector(&vector);

    assert_relative_eq!(result, expected, epsilon = 1e-10);
}

#[test]
fn test_transform_application_order1() {
    let scale = Vector3::new(1_f64, 2_f64, 3_f64);
    let axis = &Unit::from_value(Vector3::new(-1_f64, 1_f64, 3_f64));
    let angle = Radians(f64::consts::FRAC_PI_3);
    let rotation = Rotation3::from_axis_angle(&axis, angle);
    let translation = Vector3::new(2_f64, 3_f64, -4_f64);
    let transform = Transform3::new(&scale, &translation, rotation);
    let point = Vector3::new(-4_f64, -7_f64, 5_f64);
    let expected = transform.transform_point(&point);
    let result = {
        let scale_matrix = Matrix4x4::from_affine_nonuniform_scale(scale.x, scale.y, scale.z);
        let rotation_matrix = Matrix4x4::from_affine_axis_angle(&axis, angle);
        let translation_matrix = Matrix4x4::from_affine_translation(&translation);
        let matrix = translation_matrix * rotation_matrix * scale_matrix;
        let result = matrix * point.extend(1_f64);
        result.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_transform_application_order2() {
    let scale = Vector3::new(2_f64, 3_f64, 4_f64);
    let axis = &Unit::from_value(Vector3::unit_z());
    let angle = Radians(f64::consts::FRAC_PI_2);
    let rotation = Rotation3::from_axis_angle(&axis, angle);
    let translation = Vector3::new(0_f64, 3_f64, 0_f64);
    let transform = Transform3::new(&scale, &translation, rotation);
    let vector = Vector3::unit_x();
    let expected = transform.transform_vector(&vector);
    let result = {
        let scale_matrix = Matrix4x4::from_affine_nonuniform_scale(scale.x, scale.y, scale.z);
        let rotation_matrix = Matrix4x4::from_affine_axis_angle(&axis, angle);
        let translation_matrix = Matrix4x4::from_affine_translation(&translation);
        let matrix = translation_matrix * rotation_matrix * scale_matrix;
        let result = matrix * vector.extend(0_f64);
        result.contract()
    };

    assert_eq!(result, expected);
}

