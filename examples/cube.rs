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
    Scale3,
    Translation3,
    Rotation3,
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
        let translation = Vector3::new(-1_f32, -1_f32, -1_f32);
        /*
        let transform = Matrix4x4::from_affine_translation(&translation) * 
            Matrix4x4::from_affine_scale(2_f32);
        */
        let transform = {
            let scale = Vector3::from_fill(2_f32);
            Transform3::from_scale_translation(&scale, &translation)
        };
        let mut physics = World::new();
        /*
        let rigid_body = {
            let mut _rigid_body = RigidBody::default();
            let angular_frequency = 2_f32;
            let position_world = Vector3::new(0_f32, 0_f32, 0_f32);
            _rigid_body.set_rotation(&Vector3::new(0_f32, 0_f32, angular_frequency));
            _rigid_body.set_awake(false);
            _rigid_body.set_angular_damping(1_f32);
            _rigid_body.set_position(&position_world);
            // _rigid_body.set_scale(Scale3::from_scale(2_f32));
            _rigid_body
        };
        */
        let rigid_body = RigidBody::default();
        let rigid_body_instance = physics.register_body(rigid_body);
        let scene_object = SceneObjectBuilder::new(model.clone(), rigid_body_instance)
            .with_transform(&transform)
            .build();
        let active_scene = SceneBuilder::new(camera)
            .with_physics(physics)
            .with_object(scene_object)
            .build();
        let axis = Unit::from_value(Vector3::unit_z());
        let angular_frequency = 2_f64;

        Self { active_scene, axis, angular_frequency, }
    }
}

impl AppState for AppStateCube {
    fn update(&mut self, elapsed: f64) {
        /*
        let angle = Radians((self.angular_frequency * elapsed) as f32);
        let rotation_matrix = Matrix4x4::from_affine_axis_angle(&self.axis, angle);
        let old_transform = self.active_scene.get_unchecked(0).get_transform();
        let new_transform = rotation_matrix * old_transform;
        */
        /*
        let old_transform = self.active_scene.get_unchecked(0).get_transform();
        eprintln!("old_tranform = {:?}", old_transform);
        self.active_scene.run(elapsed);
        let new_transform = self.active_scene.get_unchecked(0).get_transform();
        eprintln!("new_transform = {:?}", new_transform);
        */
        let angle = Radians((self.angular_frequency * elapsed) as f32);
        let rotation = Rotation3::from_axis_angle(&self.axis, angle);
        let old_transform = self.active_scene.get_unchecked(0).get_transform();
        let new_transform = Transform3::new(
            &old_transform.scale,
            &old_transform.translation,
            rotation * old_transform.rotation
        );
        {
            let angle = Radians((self.angular_frequency * elapsed) as f32);
            let rotation_matrix = Matrix4x4::from_affine_axis_angle(&self.axis, angle);
            let old_transform = self.active_scene.get_unchecked(0).get_transform();
            let _new_transform = rotation_matrix * old_transform.to_matrix4x4();
            eprintln!("old_transform = {:?}", old_transform);
            eprintln!("_new_transform = {:?}", _new_transform);
            eprintln!("new_transform = {:?}", new_transform.to_matrix4x4());
        }
        // self.active_scene.get_mut_unchecked(0).set_transform(&new_transform);
        
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

