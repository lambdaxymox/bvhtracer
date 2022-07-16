extern crate bvhtracer;
extern crate cglinalg;


use bvhtracer::{
    Aabb,
    Ray,
};
use cglinalg::{
    Magnitude,
    Vector3,
};
use std::f32;


fn scene() -> Aabb<f32> {
    let bounds_min = Vector3::new(-1_f32, -1_f32, -1_f32);
    let bounds_max = Vector3::new(1_f32, 1_f32, 1_f32);

    Aabb::new(bounds_min, bounds_max)
}


#[test]
fn test_aabb_centroid() {
    let aabb = scene();
    let expected = Vector3::zero();
    let result = aabb.centroid();

    assert_eq!(result, expected);
}

#[test]
fn test_aabb_intersection_miss_xy_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            sphere_radius * f32::cos(ray_angle),
            sphere_radius * f32::sin(ray_angle),
            0_f32,
        );
        let ray_direction = (ray_origin - sphere_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_none());
    }
}

#[test]
fn test_aabb_intersection_miss_yz_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            0_f32,
            sphere_radius * f32::cos(ray_angle),
            sphere_radius * f32::sin(ray_angle),
        );
        let ray_direction = (ray_origin - sphere_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_none());
    }
}

#[test]
fn test_aabb_intersection_miss_zx_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            sphere_radius * f32::sin(ray_angle),
            0_f32,
            sphere_radius * f32::cos(ray_angle),
        );
        let ray_direction = (ray_origin - sphere_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_none());
    }
}

#[test]
fn test_aabb_intersection_hit_xy_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            sphere_radius * f32::cos(ray_angle),
            sphere_radius * f32::sin(ray_angle),
            0_f32,
        );
        let ray_direction = (sphere_origin - ray_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_some());
    }
}

#[test]
fn test_aabb_intersection_hit_yz_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            0_f32,
            sphere_radius * f32::cos(ray_angle),
            sphere_radius * f32::sin(ray_angle),
        );
        let ray_direction = (sphere_origin - ray_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_some());
    }
}

#[test]
fn test_aabb_intersection_hit_zx_plane() {
    let aabb = scene();
    let sphere_radius = 10_f32;
    let sphere_origin = Vector3::zero();
    let angular_positions = 64;
    for i in 0..angular_positions {
        let ray_angle = ((i as f32) / (angular_positions as f32)) * f32::consts::FRAC_2_PI;
        let ray_origin = Vector3::new(
            sphere_radius * f32::sin(ray_angle),
            0_f32,
            sphere_radius * f32::cos(ray_angle),
        );
        let ray_direction = (sphere_origin - ray_origin).normalize();
        let ray = Ray::from_origin_dir(ray_origin, ray_direction);

        assert!(aabb.intersect(&ray).is_some());
    }
}

fn aabb_intersection_towards_origin(ray_origin: Vector3<f32>, t_intersect: f32) {
    let aabb = scene();
    let sphere_origin = Vector3::zero();
    let ray_direction = (sphere_origin - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = Some(t_intersect);
    let result = aabb.intersect(&ray);

    assert_eq!(result, expected);
}

#[test]
fn test_aabb_intersection_hit_positive_x_axis() {
    aabb_intersection_towards_origin(Vector3::new(5_f32, 0_f32, 0_f32), 4_f32);
}

#[test]
fn test_aabb_intersection_hit_positive_y_axis() {
    aabb_intersection_towards_origin(Vector3::new(0_f32, 5_f32, 0_f32), 4_f32);
}

#[test]
fn test_aabb_intersection_hit_positive_z_axis() {
    aabb_intersection_towards_origin(Vector3::new(0_f32, 0_f32, 5_f32), 4_f32);
}

#[test]
fn test_aabb_intersection_hit_negative_x_axis() {
    aabb_intersection_towards_origin(Vector3::new(-5_f32, 0_f32, 0_f32), 4_f32);
}

#[test]
fn test_aabb_intersection_hit_negative_y_axis() {
    aabb_intersection_towards_origin(Vector3::new(0_f32, -5_f32, 0_f32), 4_f32);
}

#[test]
fn test_aabb_intersection_hit_negative_z_axis() {
    aabb_intersection_towards_origin(Vector3::new(0_f32, 0_f32, -5_f32), 4_f32);
}

