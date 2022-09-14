extern crate bvhtracer;
extern crate bvhtracer_demos;
extern crate cglinalg;


const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 640;


use bvhtracer::*;
use bvhtracer_demos::*;
use cglinalg::{
    Vector3,
    Radians,
    Degrees,
    Quaternion,
    Unit,
};
use std::io;


struct AppStateTwoArmadillos {
    active_scene: Scene,
}

impl AppStateTwoArmadillos {
    fn new() -> Self {
        let projection_spec = SymmetricFovSpec::new(
            Degrees(90_f32),
            1_f32,
            2_f32,
            10000_f32,
        );
        let attitude_spec = CameraAttitudeSpec::new(
            Vector3::new(0_f32, 1_f32, -2.5_f32),
            Vector3::unit_z(),
            Vector3::unit_x(),
            Vector3::unit_y(),
            Vector3::unit_z()
        );
        let camera = Camera::new(&projection_spec, &attitude_spec);
        let mesh_file = include_bytes!("assets/armadillo.tri");
        let mesh_decoder = TriMeshDecoder::new(io::Cursor::new(mesh_file));
        let mesh = mesh_decoder.read_mesh().unwrap();
        let model_builder = ModelBuilder::new();
        let model = model_builder.with_mesh(mesh).build();
        let mut objects = vec![];
        let model1 = model.clone();
        let model2 = model.clone();
        let model1_transform = Transform3::identity();
        let model2_transform = Transform3::identity();
        let mut physics = World::new();
        let rigid_body1 = {
            let mut _rigid_body = RigidBody::default();
            let position = Vector3::new(-1.3_f32, 0_f32, 0_f32);
            let orientation =  Quaternion::from_axis_angle(
                &Unit::from_value(Vector3::unit_y()),
                Radians(0_f32)
            );
            _rigid_body.set_position(&position);
            _rigid_body.set_orientation(&orientation);
            _rigid_body.set_angular_damping(1_f32);
            _rigid_body.set_can_sleep(false);
            _rigid_body.set_awake(true);
            _rigid_body
        };
        let rigid_body1_instance = physics.register_body(rigid_body1);
        let rigid_body2 = {
            let mut _rigid_body = RigidBody::default();
            let position = Vector3::new(1.3_f32, 0_f32, 0_f32);
            let orientation =  Quaternion::from_axis_angle(
                &Unit::from_value(Vector3::unit_y()),
                Radians(0_f32)
            );
            _rigid_body.set_position(&position);
            _rigid_body.set_orientation(&orientation);
            _rigid_body.set_rotation(&Vector3::new(0_f32, 0.02_f32, 0_f32));
            _rigid_body.set_angular_damping(1_f32);
            _rigid_body.set_can_sleep(false);
            _rigid_body.set_awake(true);
            _rigid_body
        };
        let rigid_body2_instance = physics.register_body(rigid_body2);
        let object1 = SceneObjectBuilder::new(model1, rigid_body1_instance)
            .with_transform(&model1_transform)
            .build();
        let object2 = SceneObjectBuilder::new(model2, rigid_body2_instance)
            .with_transform(&model2_transform)
            .build();
        objects.push(object1);
        objects.push(object2);
        let active_scene = SceneBuilder::new(camera)
            .with_physics(physics)
            .with_objects(objects)
            .build();

        Self { active_scene, }
    }
}

impl AppState for AppStateTwoArmadillos {
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
    let state = Box::new(AppStateTwoArmadillos::new());
    let elapsed = now.elapsed().unwrap();
    println!("Scene building time = {:?}", elapsed);
    let accumulator = Box::new(DepthAccumulator::new());
    let pixel_shader = Box::new(DepthMappingShader::new(80_f32, 3_f32));
    let renderer = Renderer::new(Box::new(PathTracer::new()));
    let context = init_gl("OpenGL Window", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32).unwrap();
    let mut app = App::new(context, pixel_shader, accumulator, state, renderer, SCREEN_WIDTH, SCREEN_HEIGHT);
    app.run();

    Ok(())
}

