extern crate bvhtracer;
extern crate cglinalg;


#[allow(clippy::all)]
mod gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;

mod app;
mod context;
mod state;

use bvhtracer::*;
use crate::app::*;
use crate::context::*;
use crate::state::*;

use std::io;
/*
use bvhtracer::*;
use app::*;
use context::*;
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


struct AppStateTrippyTeapots {
    active_scene: Scene,
    a: Vec<f32>,
    h: Vec<f32>,
    s: Vec<f32>,
}

impl AppStateTrippyTeapots {
    fn new() -> Self {
        let rot_mat_x = Matrix4x4::from_affine_angle_x(Radians(0.0));
        let rot_mat_z = Matrix4x4::from_affine_angle_z(Radians(2.5));
        let rot_mat = rot_mat_z * rot_mat_x;
        let _p0 = Vector3::new(-1_f32, 1_f32, 2_f32);
        let _p1 = Vector3::new(1_f32, 1_f32, 2_f32);
        let _p2 = Vector3::new(-1_f32, -1_f32, 2_f32);
        let p0 = (rot_mat * _p0.extend(1_f32)).contract();
        let p1 = (rot_mat * _p1.extend(1_f32)).contract();
        let p2 = (rot_mat * _p2.extend(1_f32)).contract();
        let focal_offset = 0_f32;
        let model_spec = PerspectiveSpec::new(
            p0.x, 
            p1.x, 
            p2.y + focal_offset,
            p0.y + focal_offset,
            p0.z, 
            10000_f32, 
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, -1.5_f32, -6.5_f32),
            Vector3::unit_z(),
            Vector3::unit_x(),
            -Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&model_spec, &attitude_spec);
        let mesh_reader = File::open("assets/teapot.obj").unwrap();
        let material_reader = File::open("assets/bricks_rgb.png").unwrap();
        let model = SimpleModelDecoder::new(mesh_reader, material_reader)
            .read_model()
            .unwrap();
        let objects = (0..16).map(|_| {
                SceneObjectBuilder::new(model.clone())
                    .with_transform(&Matrix4x4::from_affine_scale(1_f32))
                    .build()
            })
            .collect::<Vec<_>>();
        let active_scene = SceneBuilder::new(camera)
            .with_objects(objects)
            .build();
        let a = vec![0_f32; 16];
        let h = vec![
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32, 
            5_f32, 4_f32, 3_f32, 2_f32, 1_f32, 5_f32, 4_f32, 3_f32,
        ];
        let s = vec![0_f32; 16];

        Self { active_scene, a, h, s }

    }
}

impl AppState for AppStateTrippyTeapots {
    fn update(&mut self, _elapsed: f64) {
        let mut i = 0;
        for x in 0..4 {
            for y in 0..4 {
                let scale_mat = Matrix4x4::from_affine_scale(0.75_f32);
                let trans_mat = Matrix4x4::from_affine_translation(
                    &Vector3::new((x as f32 - 1.5) * 2.5, 0_f32, (y as f32 - 1.5) * 2.5)
                );
                let rot_mat = if ((x + y) & 1) == 0 {
                    Matrix4x4::from_affine_angle_x(Radians(self.a[i])) * Matrix4x4::from_affine_angle_z(Radians(self.a[i]))
                } else {
                    Matrix4x4::from_affine_translation(&Vector3::new(0_f32, self.h[i / 2], 0_f32))
                };
                self.a[i] += (((i * 13) & 7 + 2) as f32) * 0.005;
                if self.a[i] > std::f32::consts::FRAC_2_PI {
                    self.a[i] -= std::f32::consts::FRAC_2_PI;
                }
                self.s[i] -= 0.01;
                self.h[i] += self.s[i];
                if (self.s[i] < 0_f32) && (self.h[i] < 0_f32) {
                    self.s[i] = 0.2;
                }

                let new_transform = trans_mat * rot_mat * scale_mat;
                self.active_scene.get_mut_unchecked(i).set_transform(&new_transform);

                i += 1;
            }
        } 

        self.active_scene.rebuild();
    }

    fn active_scene(&self) -> &Scene {
        &self.active_scene
    }

    fn active_scene_mut(&mut self) -> &mut Scene {
        &mut self.active_scene
    }
}
*/

fn main() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateQuad::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);
    /*
    let accumulator = Box::new(IntersectionAccumulator::new(
        Vector3::from_fill(1_f32), 
        Vector3::zero()
    ));
    let pixel_shader = Box::new(IntersectionShader::new(
        Rgba::new(255, 255, 255, 255), 
        Rgba::new(0, 0, 0, 255),
    ));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    */
    /*
    let accumulator = Box::new(DepthAccumulator::new());
    let pixel_shader = Box::new(DepthMappingShader::new(80_f32, 3_f32));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    */
    /*
    let accumulator = Box::new(UvMappingAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    */
    /*
    let accumulator = Box::new(NormalMappingAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    */

    let accumulator = Box::new(TextureMaterialAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));

    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

