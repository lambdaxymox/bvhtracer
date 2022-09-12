extern crate bvhtracer;
extern crate cglinalg;

use bvhtracer::{
    BoxSpec,
    CameraAttitudeSpec,
    Camera,
    Triangle,
    TextureCoordinates,
    Normals,
    MeshBuilder,
    ModelBuilder,
    PngTextureBufferDecoder,
    TextureBuffer2D,
    Scene,
    SceneBuilder,
    SceneObjectBuilder,
    TextureMaterial,
    Rgb,
    TextureBufferDecoder,
    Ray,
    World,
    RigidBody,
    Transform3,
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


struct TestCase {
    t_centroid: f32,
    t_top_left: f32,
    t_top_right: f32,
    t_bottom_left: f32,
    t_bottom_right: f32,
    t_centroid_camera_plane: f32,
    t_top_left_camera_plane: f32,
    t_top_right_camera_plane: f32,
    t_bottom_left_camera_plane: f32,
    t_bottom_right_camera_plane: f32,
    uv_dimensions: Vector2<f32>,
    uv_top_left: Vector2<f32>,
    uv_top_right: Vector2<f32>,
    uv_bottom_left: Vector2<f32>,
    uv_bottom_right: Vector2<f32>,
}


fn scene() -> Scene {
    let projection_spec = BoxSpec::new(
        -1_f32, 
         1_f32, 
        -1_f32, 
         1_f32, 
         1_f32, 
         100_f32, 
    );
    let attitude_spec = CameraAttitudeSpec::new(
         Vector3::new(0_f32, 0_f32, 2_f32),
        -Vector3::unit_z(),
         Vector3::unit_x(),
         Vector3::unit_y(),
        -Vector3::unit_z()
    );
    let camera = Camera::new(&projection_spec, &attitude_spec);
    let mesh = MeshBuilder::new()
        .with_primitive(
            Triangle::new(
                Vector3::new(-1.0, -1.0, 0.0), 
                Vector3::new( 1.0,  1.0, 0.0), 
                Vector3::new(-1.0,  1.0, 0.0),
            ),
            TextureCoordinates::from([
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 1.0),
                Vector2::new(0.0, 1.0),
            ]),
            Normals::from([
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, 1.0),
            ])
        )
        .with_primitive(
            Triangle::new(
                Vector3::new(-1.0, -1.0, 0.0),
                Vector3::new( 1.0, -1.0, 0.0),
                Vector3::new( 1.0,  1.0, 0.0),
            ),
            TextureCoordinates::from([
                Vector2::new(0.0, 0.0),
                Vector2::new(1.0, 0.0),
                Vector2::new(1.0, 1.0),
            ]),
            Normals::from([
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 0.0, 1.0),
            ])
        )
        .build();
    let material = {
        let material_reader = File::open("assets/bricks_rgb.png").unwrap();
        let decoder: PngTextureBufferDecoder<Rgb<u8>, _> = PngTextureBufferDecoder::new(material_reader);
        let texture = decoder.read_texture().unwrap();
        TextureMaterial::new(texture)
    };
    let model = ModelBuilder::new()
        .with_mesh(mesh)
        .with_texture(material)
        .build();
    let mut physics = World::new();
    let rigid_body = {
        let mut _rigid_body = RigidBody::default();
        _rigid_body.set_rotation(&Vector3::new(0_f32, 0_f32, 2_f32));
        _rigid_body
    };
    let rigid_body_instance = physics.register_body(rigid_body);
    let scene_object = SceneObjectBuilder::new(model, rigid_body_instance)
        // .with_transform(&Matrix4x4::identity())
        .with_transform(&Transform3::identity())
        .build();
    let active_scene = SceneBuilder::new(camera)
        .with_physics(physics)
        .with_object(scene_object)
        .build();

    active_scene
}

fn test_case() -> TestCase {
    let ray_origin = Vector3::new(0.0, 0.0, 2.0);
    let top_left = Vector3::new(-1.0, 1.0, 0.0);
    let top_right = Vector3::new(1.0, 1.0, 0.0);
    let bottom_left = Vector3::new(-1.0, -1.0, 0.0);
    let bottom_right = Vector3::new(1.0, -1.0, 0.0);
    let ray_top_left = {
        let ray_direction = (top_left - ray_origin).normalize();
        Ray::from_origin_dir(ray_origin, ray_direction)
    };
    let ray_top_right = {
        let ray_direction = (top_right - ray_origin).normalize();
        Ray::from_origin_dir(ray_origin, ray_direction)
    };
    let ray_bottom_left = {
        let ray_direction = (bottom_left - ray_origin).normalize();
        Ray::from_origin_dir(ray_origin, ray_direction)
    };
    let ray_bottom_right = {
        let ray_direction = (bottom_right - ray_origin).normalize();
        Ray::from_origin_dir(ray_origin, ray_direction)
    };
    let t_centroid = 2_f32;
    let t_top_left = f32::sqrt(6_f32);
    let t_top_right = f32::sqrt(6_f32);
    let t_bottom_left = f32::sqrt(6_f32);
    let t_bottom_right = f32::sqrt(6_f32);
    let t_top_left_camera_plane = f32::sqrt(6_f32) / 2_f32;
    let t_top_right_camera_plane = f32::sqrt(6_f32) / 2_f32;
    let t_bottom_left_camera_plane = f32::sqrt(6_f32) / 2_f32;
    let t_bottom_right_camera_plane = f32::sqrt(6_f32) / 2_f32;
    let t_centroid_camera_plane = 1_f32;
    let top_left_camera_plane = ray_top_left.interpolate(t_top_left_camera_plane);
    let top_right_camera_plane = ray_top_right.interpolate(t_top_right_camera_plane);
    let bottom_left_camera_plane = ray_bottom_left.interpolate(t_bottom_left_camera_plane);
    let bottom_right_camera_plane = ray_bottom_right.interpolate(t_bottom_right_camera_plane);
    let uv_dimensions = Vector2::new(
        (top_right_camera_plane.x - top_left_camera_plane.x) / 2_f32,
        (top_left_camera_plane.y - bottom_left_camera_plane.y) / 2_f32,
    );
    let uv_top_left = Vector2::new(
        (top_left_camera_plane.x - top_left.x) / 2_f32,
        (top_left.y - top_left_camera_plane.y) / 2_f32,
    );
    let uv_top_right = Vector2::new(uv_top_left.x + uv_dimensions.x, uv_top_left.y);
    let uv_bottom_left = Vector2::new(uv_top_left.x, uv_top_left.y + uv_dimensions.y);
    let uv_bottom_right = uv_top_left + uv_dimensions;

    TestCase {
        t_centroid,
        t_top_left,
        t_top_right,
        t_bottom_left,
        t_bottom_right,
        t_centroid_camera_plane,
        t_top_left_camera_plane,
        t_top_right_camera_plane,
        t_bottom_left_camera_plane,
        t_bottom_right_camera_plane,
        uv_dimensions,
        uv_top_left,
        uv_top_right,
        uv_bottom_left,
        uv_bottom_right,
    }
}

#[test]
fn test_quad_depth_centroid() {
    let scene = scene();
    let test_case = test_case();
    let expected = Some(test_case.t_centroid);
    let ray = scene.active_camera().get_ray_world(0.5, 0.5);
    let result = scene.intersect(&ray).map(|itr| itr.interaction.t);

    assert_eq!(result, expected);
}

#[test]
fn test_quad_depth_top_left() {
    let scene = scene();
    let test_case = test_case();
    let expected = Some(test_case.t_top_left);
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_left.x, test_case.uv_top_left.y);
    let result = scene.intersect(&ray).map(|itr| itr.interaction.t);

    assert_eq!(result, expected);
}

#[test]
fn test_quad_depth_top_right() {
    let scene = scene();
    let test_case = test_case();
    let expected = Some(test_case.t_top_right);
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_right.x, test_case.uv_top_right.y);
    let result = scene.intersect(&ray).map(|itr| itr.interaction.t);

    assert_eq!(result, expected);
}

#[test]
fn test_quad_depth_bottom_left() {
    let scene = scene();
    let test_case = test_case();
    let expected = Some(test_case.t_bottom_left);
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_bottom_left.x, test_case.uv_bottom_left.y);
    let result = scene.intersect(&ray).map(|itr| itr.interaction.t);

    assert_eq!(result, expected);
}

#[test]
fn test_quad_depth_bottom_right() {
    let scene = scene();
    let test_case = test_case();
    let expected = Some(test_case.t_bottom_right);
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_bottom_right.x, test_case.uv_bottom_right.y);
    let result = scene.intersect(&ray).map(|itr| itr.interaction.t);

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_quad_depth_centroid() {
    let scene = scene();
    let test_case = test_case();
    let expected = 1_f32;
    let ray = scene.active_camera().get_ray_world(0.5, 0.5);
    let result = ray.interpolate(test_case.t_centroid_camera_plane).z;

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_quad_depth_top_left() {
    let scene = scene();
    let test_case = test_case();
    let expected = 1_f32;
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_left.x, test_case.uv_top_left.y);
    let result = ray.interpolate(test_case.t_top_left_camera_plane).z;

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_quad_depth_top_right() {
    let scene = scene();
    let test_case = test_case();
    let expected = 1_f32;
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_right.x, test_case.uv_top_right.y);
    let result = ray.interpolate(test_case.t_top_left_camera_plane).z;

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_quad_depth_bottom_left() {
    let scene = scene();
    let test_case = test_case();
    let expected = 1_f32;
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_bottom_left.x, test_case.uv_bottom_left.y);
    let result = ray.interpolate(test_case.t_top_left_camera_plane).z;

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_quad_depth_bottom_right() {
    let scene = scene();
    let test_case = test_case();
    let expected = 1_f32;
    let ray = scene
        .active_camera()
        .get_ray_world(test_case.uv_bottom_right.x, test_case.uv_bottom_right.y);
    let result = ray.interpolate(test_case.t_top_left_camera_plane).z;

    assert_eq!(result, expected);
}

#[test]
fn test_camera_plane_solid_angle() {
    let scene = scene();
    let test_case = test_case();
    let expected = test_case.uv_dimensions;
    let ray_top_left = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_left.x, test_case.uv_top_left.y);
    let ray_top_right = scene
        .active_camera()
        .get_ray_world(test_case.uv_top_right.x, test_case.uv_top_right.y);
    let ray_bottom_left = scene
        .active_camera()
        .get_ray_world(test_case.uv_bottom_left.x, test_case.uv_bottom_left.y);
    let result = {
        let top_left = ray_top_left.interpolate(test_case.t_top_left_camera_plane);
        let top_right = ray_top_right.interpolate(test_case.t_top_right_camera_plane);
        let bottom_left = ray_bottom_left.interpolate(test_case.t_bottom_left_camera_plane);
        let du = (top_right.x - top_left.x) / 2_f32;
        let dv = (top_left.y - bottom_left.y) / 2_f32;

        Vector2::new(du, dv)
    };

    assert_eq!(result, expected);
}


#[test]
fn test_scene_intersection_entire_viewport() {
    let scene = scene();
    let test_case = test_case();
    let width = 640;
    let height = 640;
    for y in 0..height {
        for x in 0..width {
            let u = x as f32 / width as f32;
            let v = y as f32 / height as f32;
            let ray = scene.active_camera().get_ray_world(u, v);
            let u_in_solid_angle = u >= test_case.uv_top_left.x && u <= test_case.uv_top_right.x;
            let v_in_solid_angle = v >= test_case.uv_top_left.y && v <= test_case.uv_bottom_left.y;
            let uv_in_solid_angle = u_in_solid_angle && v_in_solid_angle;
            let expected = if uv_in_solid_angle { true } else { false };
            let result = scene.intersect(&ray);
            assert_eq!(
                result.is_some(), expected, 
                "u = {}; v = {}; result = {:?}; u_in_solid_angle = {}; v_in_solid_angle = {}", 
                u, v, result, u_in_solid_angle, v_in_solid_angle
            );
        }
    }
}

