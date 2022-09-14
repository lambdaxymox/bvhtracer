extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Vector3,
    Degrees,
    Magnitude,
    Quaternion,
};
use std::io;


struct AppStateCube {
    active_scene: Scene,
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
        let transform = Transform3::from_scale_translation(
            &Vector3::from_fill(2_f32), 
            &Vector3::new(-1_f32, -1_f32, -1_f32)
        );
        let mut physics = World::new();
        let rigid_body = {
            let mut _rigid_body = RigidBody::default();
            let angular_frequency = 2_f32;
            let position_world = Vector3::new(0_f32, 0_f32, 0_f32);
            _rigid_body.set_position(&position_world);
            _rigid_body.set_orientation(&Quaternion::unit_z());
            _rigid_body.set_rotation(&Vector3::new(0_f32, 0_f32, angular_frequency));
            _rigid_body.set_awake(true);
            _rigid_body.set_angular_damping(1_f32);
            _rigid_body
        };
        let rigid_body_instance = physics.register_body(rigid_body);
        let scene_object = SceneObjectBuilder::new(model.clone(), rigid_body_instance)
            .with_transform(&transform)
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_physics(physics)
            .with_object(scene_object)
            .build();

        Self { active_scene, }
    }
}

impl AppState for AppStateCube {
    fn update(&mut self, elapsed: f64) {
        self.active_scene.run(elapsed);
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

