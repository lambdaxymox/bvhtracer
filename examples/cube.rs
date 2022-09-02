extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Matrix4x4,
    Vector3,
    Radians,
    Unit,
    Degrees,
    Magnitude,
};
use std::io;


struct AppStateCube {
    active_scene: Scene,
    axis: Unit<Vector3<f32>>,
    angular_frequency: f64,
}

impl AppStateCube {
    fn new() -> Self {
        let projection_spec = SymmetricFovSpec::new(
            Degrees(90_f32),
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
        let camera = Camera::new(&projection_spec, &attitude_spec);
        let mesh_file = include_bytes!("assets/cube.obj");
        let mesh_reader = io::Cursor::new(mesh_file);
        let material_file = include_bytes!("assets/bricks_rgb.png");
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


pub fn main() -> io::Result<()> {
    use std::time::SystemTime;
    println!("Building scene.");
    let now = SystemTime::now();
    let state = Box::new(AppStateCube::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);
    let accumulator = Box::new(NormalMappingAccumulator::new());
    let pixel_shader = Box::new(RadianceToRgbShader::new());
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

