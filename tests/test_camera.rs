extern crate bvhtracer;

use bvhtracer::{
    PerspectiveProjection,
    PerspectiveSpec,
    CameraAttitudeSpec,
    Camera,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};


fn camera() -> Camera<f64, PerspectiveProjection<f64>> {
    let model_spec = PerspectiveSpec::new(
        -4_f64, 
        4_f64, 
        -3_f64, 
        3_f64, 
        1_f64, 
        100_f64
    );
    let attitude_spec = CameraAttitudeSpec::new(
        Vector3::new(0_f64, 0_f64, -5_f64),
        Vector3::unit_z(),
        Vector3::unit_x(),
        Vector3::unit_y(),
        Vector3::unit_z()
    );

    Camera::new(&model_spec, &attitude_spec)
}

#[test]
fn test_camera_top_left_corner_eye() {
    let camera = camera();
    let expected = Vector3::new(-4_f64, 3_f64, -1_f64);
    let result = camera.top_left_eye();

    assert_eq!(result, expected);
}

#[test]
fn test_camera_top_left_corner_world() {
    let camera = camera();
    let expected = Vector3::new(-4_f64, 3_f64, -4_f64);
    let result = {
        let top_left_eye = camera.top_left_eye().extend(1_f64);
        let top_left_world = camera.view_matrix_inv() * top_left_eye;
        top_left_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_bottom_left_corner_eye() {
    let camera = camera();
    let expected = Vector3::new(-4_f64, -3_f64, -1_f64);
    let result = camera.bottom_left_eye();

    assert_eq!(result, expected);
}

#[test]
fn test_camera_bottom_left_corner_world() {
    let camera = camera();
    let expected = Vector3::new(-4_f64, -3_f64, -4_f64);
    let result = {
        let bottom_left_eye = camera.bottom_left_eye().extend(1_f64);
        let bottom_left_world = camera.view_matrix_inv() * bottom_left_eye;
        bottom_left_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_top_right_corner_eye() {
    let camera = camera();
    let expected = Vector3::new(4_f64, 3_f64, -1_f64);
    let result = camera.top_right_eye();

    assert_eq!(result, expected);
}

#[test]
fn test_camera_top_right_corner_world() {
    let camera = camera();
    let expected = Vector3::new(4_f64, 3_f64, -4_f64);
    let result = {
        let top_right_eye = camera.top_right_eye().extend(1_f64);
        let top_right_world = camera.view_matrix_inv() * top_right_eye;
        top_right_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_bottom_right_corner_eye() {
    let camera = camera();
    let expected = Vector3::new(4_f64, -3_f64, -1_f64);
    let result = camera.bottom_right_eye();

    assert_eq!(result, expected);
}

#[test]
fn test_camera_bottom_right_corner_world() {
    let camera = camera();
    let expected = Vector3::new(4_f64, 3_f64, -4_f64);
    let result = {
        let bottom_right_eye = camera.bottom_right_eye().extend(1_f64);
        let bottom_right_world = camera.view_matrix_inv() * bottom_right_eye;
        bottom_right_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_get_ray_top_left_corner() {
    let camera = camera();
    let ray_origin = Vector3::new(0_f64, 0_f64, -5_f64);
    let ray_direction = (Vector3::new(-4_f64, 3_f64, -4_f64) - ray_origin).normalize();
    let expected = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = camera.get_ray_world(0_f64, 0_f64);

    assert_eq!(result, expected);
}

#[test]
fn test_camera_get_ray_bottom_left_corner() {
    let camera = camera();
    let ray_origin = Vector3::new(0_f64, 0_f64, -5_f64);
    let ray_direction = (Vector3::new(-4_f64, -3_f64, -4_f64) - ray_origin).normalize();
    let expected = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = camera.get_ray_world(0_f64, 1_f64);

    assert_eq!(result, expected);
}

#[test]
fn test_camera_get_ray_top_right_corner() {
    let camera = camera();
    let ray_origin = Vector3::new(0_f64, 0_f64, -5_f64);
    let ray_direction = (Vector3::new(4_f64, 3_f64, -4_f64) - ray_origin).normalize();
    let expected = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = camera.get_ray_world(1_f64, 0_f64);
 
    assert_eq!(result, expected);
}

#[test]
fn test_camera_get_ray_bottom_right_corner() {
    let camera = camera();
    let ray_origin = Vector3::new(0_f64, 0_f64, -5_f64);
    let ray_direction = (Vector3::new(4_f64, -3_f64, -4_f64) - ray_origin).normalize();
    let expected = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = camera.get_ray_world(1_f64, 1_f64);

    assert_eq!(result, expected);
}

#[test]
fn test_camera_get_ray_centroid() {
    let camera = camera();
    let ray_origin = Vector3::new(0_f64, 0_f64, -5_f64);
    let ray_direction = Vector3::new(0_f64, 0_f64, 1_f64);
    let expected = Ray::from_origin_dir(ray_origin, ray_direction);
    let result = camera.get_ray_world(0.5_f64, 0.5_f64);

    assert_eq!(result, expected);
}

#[test]
fn test_camera_forward_world() {
    let camera = camera();
    let expected = Vector3::unit_z();
    let result = {
        let forward_eye = camera.forward_axis_eye();
        let forward_world = camera.view_matrix_inv() * forward_eye.extend(0_f64);
        forward_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_right_axis_world() {
    let camera = camera();
    let expected = Vector3::unit_x();
    let result = {
        let right_eye = camera.right_axis_eye();
        let right_world = camera.view_matrix_inv() * right_eye.extend(0_f64);
        right_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_up_axis_world() {
    let camera = camera();
    let expected = -Vector3::unit_y();
    let result = {
        let up_eye = camera.up_axis_eye();
        let up_world = camera.view_matrix_inv() * up_eye.extend(0_f64);
        up_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_z_axis_world() {
    let camera = camera();
    let expected = -Vector3::unit_z();
    let result = {
        let z_axis_eye = Vector3::unit_z();
        let z_axis_world = camera.view_matrix_inv() * z_axis_eye.extend(0_f64);
        z_axis_world.contract()
    };
    eprintln!("view_matrix = {:?}", camera.view_matrix());

    assert_eq!(result, expected);
}


#[test]
fn test_camera_origin_eye_to_world() {
    let camera = camera();
    let expected = Vector3::new(0_f64, 0_f64, -5_f64);
    let result = {
        let origin_eye = Vector3::zero();
        let origin_world = camera.view_matrix_inv() * origin_eye.extend(1_f64);
        origin_world.contract()
    };

    assert_eq!(result, expected);
}

#[test]
fn test_camera_origin_world_to_eye() {
    let camera = camera();
    let expected = Vector3::zero();
    let result = {
        let origin_world = Vector3::new(0_f64, 0_f64, -5_f64);
        let origin_eye = camera.view_matrix() * origin_world.extend(1_f64);
        origin_eye.contract()
    };

    assert_eq!(result, expected);
}

