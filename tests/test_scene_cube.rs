extern crate bvhtracer;
extern crate cglinalg;
extern crate approx;

use bvhtracer::{
    Scene,
    Camera,
    CameraAttitudeSpec,
    SimpleModelDecoder,
    ModelDecoder,
    SceneObjectBuilder,
    SceneBuilder,
    PerspectiveSpec,
    Triangle,
    Normals,
    Ray,
};
use approx::{
    assert_relative_eq,
};
use cglinalg::{
    Vector2,
    Vector3,
    Matrix4x4,
    Magnitude,
};
use std::fs::{
    File,
};


fn scene() -> Scene {
    let model_spec = PerspectiveSpec::new(
        -1_f32, 
        1_f32, 
        -1_f32, 
        1_f32, 
        1_f32, 
        100_f32, 
    );
    let position = Vector3::new(0_f32, 4_f32, 0_f32);
    let forward = (Vector3::zero() - position).normalize();
    let attitude_spec = CameraAttitudeSpec::new(
        position,
        forward,
        -Vector3::unit_x(),
        Vector3::unit_z(),
        -forward
    );
    let camera = Camera::new(&model_spec, &attitude_spec);
    let mesh_reader = File::open("assets/cube.obj").unwrap();
    let material_reader = File::open("assets/bricks_rgb.png").unwrap();
    let model = SimpleModelDecoder::new(mesh_reader, material_reader)
        .read_model()
        .unwrap();
    let transform = {
        let translation = Matrix4x4::from_affine_translation(
            &Vector3::new(-1_f32, -1_f32, -1_f32)
        );
        let scale = Matrix4x4::from_affine_scale(2_f32);
        translation * scale
    };
    let scene_object = SceneObjectBuilder::new(model.clone())
        .with_transform(&transform)
        .build();
    let active_scene = SceneBuilder::new(camera)
        .with_object(scene_object)
        .build();

    active_scene
}

#[test]
fn test_scene_intersection_lands1() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(0.5, 1.0, -0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);

    assert!(scene.intersect(&ray).is_some());
    /*
    let mesh = {
        let model = scene.get_unchecked(0).model().model();
        let borrow = model.borrow();
        borrow.mesh().clone()
    };
    let transform = scene.get_unchecked(0).get_transform();
    let transformed_mesh = mesh.primitives().iter().map(|tri| {
            let vertex0 = (transform * tri.vertices[0].extend(1_f32)).contract();
            let vertex1 = (transform * tri.vertices[1].extend(1_f32)).contract();
            let vertex2 = (transform * tri.vertices[2].extend(1_f32)).contract();
            Triangle::new(vertex0, vertex1, vertex2)
        })
        .collect::<Vec<_>>();
    let transformed_normals = mesh.normals().iter().map(|normals| {
            let normals0 = (transform * normals[0].extend(0_f32)).contract().normalize();
            let normals1 = (transform * normals[1].extend(0_f32)).contract().normalize();
            let normals2 = (transform * normals[2].extend(0_f32)).contract().normalize();
            Normals::from([normals0, normals1, normals2])
        })
        .collect::<Vec<_>>();
    eprintln!("transform = {:?}", transform);
    for i in 0..transformed_mesh.len() {
        eprintln!("mesh[{}] = {:?};\nnormals[{}] = {:?}", i, transformed_mesh[i], i, transformed_normals[i]);
    }
    unimplemented!()
    */
}

#[test]
fn test_scene_intersection_lands2() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(-0.5, 1.0, 0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);

    assert!(scene.intersect(&ray).is_some());
    /*
    let mesh = {
        let model = scene.get_unchecked(0).model().model();
        let borrow = model.borrow();
        borrow.mesh().clone()
    };
    let transform = scene.get_unchecked(0).get_transform();
    let transformed_mesh = mesh.primitives().iter().map(|tri| {
            let vertex0 = (transform * tri.vertices[0].extend(1_f32)).contract();
            let vertex1 = (transform * tri.vertices[1].extend(1_f32)).contract();
            let vertex2 = (transform * tri.vertices[2].extend(1_f32)).contract();
            Triangle::new(vertex0, vertex1, vertex2)
        })
        .collect::<Vec<_>>();
    let transformed_normals = mesh.normals().iter().map(|normals| {
            let normals0 = (transform * normals[0].extend(0_f32)).contract().normalize();
            let normals1 = (transform * normals[1].extend(0_f32)).contract().normalize();
            let normals2 = (transform * normals[2].extend(0_f32)).contract().normalize();
            Normals::from([normals0, normals1, normals2])
        })
        .collect::<Vec<_>>();
    eprintln!("transform = {:?}", transform);
    for i in 0..transformed_mesh.len() {
        eprintln!("mesh[{}] = {:?};\nnormals[{}] = {:?}", i, transformed_mesh[i], i, transformed_normals[i]);
    }
    unimplemented!()
    */
}

#[test]
fn test_scene_intersection1_t() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(0.5, 1.0, -0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = f32::sqrt(19_f32 / 2_f32);
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.interaction.t;

    assert_relative_eq!(result, expected, epsilon = 1e-7);
}

#[test]
fn test_scene_intersection1_instance_index() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(0.5, 1.0, -0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = 0;
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.instance_primitive.instance_index();

    assert_eq!(result, expected);
}

#[test]
fn test_scene_intersection1_primitive_index() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(0.5, 1.0, -0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = 6;
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.instance_primitive.primitive_index();

    assert_eq!(result, expected);
}

#[test]
fn test_scene_intersection2() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(-0.5, 1.0, 0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = f32::sqrt(19_f32 / 2_f32);
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.interaction.t;

    assert_relative_eq!(result, expected, epsilon = 1e-7);
}

#[test]
fn test_scene_intersection2_instance_index() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(-0.5, 1.0, 0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = 0;
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.instance_primitive.instance_index();

    assert_eq!(result, expected);
}

#[test]
fn test_scene_intersection2_primitive_index() {
    let scene = scene();
    let ray_origin = Vector3::new(0_f32, 4_f32, 0_f32);
    let target = Vector3::new(-0.5, 1.0, 0.5);
    let ray_direction = (target - ray_origin).normalize();
    let ray = Ray::from_origin_dir(ray_origin, ray_direction);
    let expected = 9;
    let intersection = scene.intersect(&ray).unwrap();
    let result = intersection.instance_primitive.primitive_index();

    assert_eq!(result, expected);
}