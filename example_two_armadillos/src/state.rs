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


pub struct AppStateTwoArmadillos {
    angle: f32,
    active_scene: Scene,
}

impl AppStateTwoArmadillos {
    pub fn new() -> Self {
        let focal_offset = 0.25_f32;
        let model_spec = PerspectiveSpec::new(
            -1_f32, 
            1_f32, 
            -1_f32 + focal_offset, 
            1_f32 + focal_offset, 
            2.0_f32,
            10000_f32,
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 0_f32, -4.5_f32),
            Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&model_spec, &attitude_spec);
        let mesh_file = include_bytes!("../assets/armadillo.tri");
        let mesh_decoder = TriMeshDecoder::new(io::Cursor::new(mesh_file));
        let mesh = mesh_decoder.read_mesh().unwrap();
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_mesh(mesh).build();
        let mut objects = vec![];
        let model1_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(-1.3_f32, 0_f32, 0_f32)
        );
        let model2_transform = Matrix4x4::from_affine_translation(
            &Vector3::new(1.3_f32, 0_f32, 0_f32)
        );
        let model1 = model.clone();
        let model2 = model.clone();
        let object1 = SceneObjectBuilder::new(model1)
            .with_transform(&model1_transform)
            .build();
        let object2 = SceneObjectBuilder::new(model2)
            .with_transform(&model2_transform)
            .build();
        objects.push(object1);
        objects.push(object2);
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();
        let angle = 0_f32;

        Self { angle, active_scene, }
    }
}

impl AppState for AppStateTwoArmadillos {
    fn update(&mut self, _elapsed: f64) {
        self.angle += 0.01;
        if self.angle > std::f32::consts::FRAC_2_PI {
            self.angle -= std::f32::consts::FRAC_2_PI;
        }
        self.active_scene.get_mut_unchecked(0).set_transform(
            &Matrix4x4::from_affine_translation(&Vector3::new(-1.3_f32, 0_f32, 0_f32))
        );
        self.active_scene.get_mut_unchecked(1).set_transform(&(
            Matrix4x4::from_affine_translation(&Vector3::new(1.3_f32, 0_f32, 0_f32)) *
            Matrix4x4::from_affine_angle_y(Radians(self.angle))
        ));

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}

