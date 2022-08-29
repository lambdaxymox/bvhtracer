extern crate bvhtracer;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use crate::app::*;
use crate::context::*;
use cglinalg::{
    Matrix4x4,
    Vector2,
    Vector3,
    Radians,
    Degrees,
    Unit,
};
use glfw::{
    Action, 
    Context, 
    Key
};
use gl::types::{
    GLuint, 
    GLfloat
};
use rand::{
    Rng,
    SeedableRng, 
};
use rand_isaac::{
    IsaacRng,
};
use std::fs::{
    File,
};
use std::io;
use std::mem;
use std::ptr;
use std::ffi::{
    c_void,
};


impl GpuTextureBuffer2D for TextureBuffer2D<Rgba<u8>, Vec<u8>> {
    fn format(&self) -> GpuTextureFormat { 
        GpuTextureFormat::Rgba8
    }

    fn width(&self) -> usize {
        TextureBuffer2D::width(self)
    }

    fn height(&self) -> usize {
        TextureBuffer2D::height(self)
    }

    fn dimensions(&self) -> (usize, usize) {
        TextureBuffer2D::dimensions(self)
    }

    fn as_ptr(&self) -> *const u8 {
        TextureBuffer2D::as_ptr(self) as *const u8
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        TextureBuffer2D::as_mut_ptr(self) as *mut u8
    }
}


pub struct AppStateCube {
    active_scene: Scene,
    axis: Unit<Vector3<f32>>,
    angular_frequency: f64,
}

impl AppStateCube {
    pub fn new() -> Self {
        use cglinalg::Magnitude;
        /*
        let model_spec = PerspectiveFovSpec::new(
            Degrees(90_f32), 
            1_f32, 
            0.001_f32, 
            100_f32, 
        );
        */
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
        let mesh_file = include_bytes!("../assets/cube.obj");
        let mesh_reader = io::Cursor::new(mesh_file);
        let material_file = include_bytes!("../assets/bricks_rgb.png");
        let material_reader = io::Cursor::new(material_file);
        let model = SimpleModelDecoder::new(mesh_reader, material_reader)
            .read_model()
            .unwrap();
        let scene_object = SceneObjectBuilder::new(model.clone())
            .with_transform(&(
                Matrix4x4::from_affine_translation(&Vector3::new(-1_f32, -1_f32, -1_f32)) * 
                Matrix4x4::from_affine_scale(2_f32))
            )
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_object(scene_object)
            .build();
        let axis = Unit::from_value(Vector3::unit_z());
        let angular_frequency = 1_f64;

        Self { active_scene, axis, angular_frequency, }
    }
}

impl AppState for AppStateCube {
    fn update(&mut self, elapsed: f64) {
        println!("elapsed = {}", elapsed);
        let angle = Radians((self.angular_frequency * elapsed) as f32);
        let rotation_matrix = Matrix4x4::from_affine_axis_angle(&self.axis, angle);
        let old_transform = self.active_scene.get_unchecked(0).get_transform();
        let new_transform = rotation_matrix * old_transform;
        self.active_scene.get_mut_unchecked(0).set_transform(&new_transform);
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}

